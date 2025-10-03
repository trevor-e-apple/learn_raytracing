use rand::{Rng, rngs::ThreadRng};

use crate::{
    aabb::{Aabb, hit_aabb},
    hit_record::HitRecord,
    quad::{Quad, hit_quad},
    ray::Ray,
    sphere::{Sphere, hit_sphere},
};

#[derive(Clone)]
struct NodeData {
    left: usize,
    right: usize,
    bbox: Aabb,
}

enum BvhNode {
    Node(NodeData),
    Object(Hittable),
}

pub struct Hittables {
    objects: Vec<Hittable>,
    bvh_nodes: Vec<BvhNode>,
    root: Option<usize>,
    rng: ThreadRng,
}

impl Hittables {
    pub fn new() -> Self {
        Self {
            objects: vec![],
            bvh_nodes: vec![],
            root: None,
            rng: ThreadRng::default(),
        }
    }

    pub fn add_object(&mut self, object: Hittable) -> usize {
        // self.bbox = Aabb::from_boxes(&self.bbox, &s.bounding_box);
        let handle = self.objects.len();
        self.objects.push(object);

        // Set root to None since we need to reconstruct it now
        self.root = None;

        handle
    }

    fn add_node(&mut self, node: BvhNode) -> usize {
        let handle = self.bvh_nodes.len();
        self.bvh_nodes.push(node);
        handle
    }

    /// Use a bounding volume hierarchy to find the closest hit record.
    /// The function takes a mutable in order to allow us to construct a bvh just-in-time while
    /// not allowing the function to be called on data in a transient state.
    pub fn get_hit_record(&mut self, ray_in: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
        // Get the bvh root
        let bvh_root = match self.root {
            Some(root) => root,
            None => {
                // Construct BVH if it does not already exist

                // Clear bvh_nodes
                self.bvh_nodes.clear();

                // Initialize stack
                let all_contained_objects = {
                    let mut all_contained_objects: Vec<Hittable> =
                        Vec::<Hittable>::with_capacity(self.objects.len());
                    for object in &self.objects {
                        all_contained_objects.push(object.clone());
                    }
                    all_contained_objects
                };
                let all_bbox = bbox_from_objects(&all_contained_objects);
                self.add_node(BvhNode::Node(NodeData {
                    left: 0,
                    right: 0,
                    bbox: all_bbox,
                }));

                let mut stack: Vec<(usize, Vec<Hittable>)> = vec![(0, all_contained_objects)];

                loop {
                    match stack.pop() {
                        Some((node_handle, mut contained_objects)) => {
                            let expected_left_handle = self.bvh_nodes.len();
                            let expected_right_handle = self.bvh_nodes.len() + 1;

                            // Need to limit the mutable borrow so we handle this update separately
                            match &mut self.bvh_nodes[node_handle] {
                                BvhNode::Node(node_data) => {
                                    if contained_objects.len() == 1 {
                                        node_data.left = expected_left_handle;
                                        node_data.right = expected_left_handle;
                                        // No need to put back on stack
                                    } else if contained_objects.len() == 2 {
                                        node_data.left = expected_left_handle;
                                        node_data.right = expected_right_handle;
                                        // No need to put back on stack
                                    } else {
                                        node_data.left = expected_left_handle;
                                        node_data.right = expected_right_handle;
                                    }
                                }
                                BvhNode::Object(_) => {
                                    panic!("Object on the BVH construction stack")
                                }
                            }

                            // Now add the nodes themselves
                            if contained_objects.len() == 1 {
                                self.add_node(BvhNode::Object(contained_objects[0].clone()));
                            } else if contained_objects.len() == 2 {
                                self.add_node(BvhNode::Object(contained_objects[0].clone()));
                                self.add_node(BvhNode::Object(contained_objects[1].clone()));
                            } else {
                                // Sort the objects by a random axis.
                                // The longest axis method has an issue if most objects are on the same plane,
                                // then no real sorting occurs and the bounding boxes don't decrease in size.
                                {
                                    let choice = self.rng.random_range(0..3);
                                    if choice == 0 {
                                        contained_objects.sort_by(|a, b| {
                                            a.get_bounding_box()
                                                .x0
                                                .total_cmp(&b.get_bounding_box().x0)
                                        })
                                    } else if choice == 1 {
                                        contained_objects.sort_by(|a, b| {
                                            a.get_bounding_box()
                                                .y0
                                                .total_cmp(&b.get_bounding_box().y0)
                                        })
                                    } else {
                                        contained_objects.sort_by(|a, b| {
                                            a.get_bounding_box()
                                                .z0
                                                .total_cmp(&b.get_bounding_box().z0)
                                        })
                                    }
                                }

                                // Put half of the objects in the left and half the objects in the right
                                let (left_objects, right_objects) =
                                    contained_objects.split_at(contained_objects.len() / 2);
                                let left_objects = left_objects.to_vec();
                                let right_objects = right_objects.to_vec();

                                self.add_node(BvhNode::Node(NodeData {
                                    left: 0,
                                    right: 0,
                                    bbox: bbox_from_objects(&left_objects),
                                }));
                                self.add_node(BvhNode::Node(NodeData {
                                    left: 0,
                                    right: 0,
                                    bbox: bbox_from_objects(&right_objects),
                                }));

                                // Add to the stack
                                stack.push((expected_left_handle, left_objects));
                                stack.push((expected_right_handle, right_objects));
                            }
                        }
                        None => break,
                    }
                }

                self.root = Some(0); // Update so we don't reconstruct again
                0 // return the root's handle
            }
        };

        let closest_record = {
            let mut stack: Vec<usize> = vec![bvh_root];

            let mut closest_record: Option<HitRecord> = None;
            let mut closest = tmax;

            loop {
                let current_node_handle = match stack.pop() {
                    Some(current) => current,
                    None => break,
                };

                let current_node = &self.bvh_nodes[current_node_handle];

                match current_node {
                    BvhNode::Node(node_data) => {
                        if hit_aabb(&node_data.bbox, ray_in, tmin, closest) {
                            // Add both left and right to the stack
                            stack.push(node_data.left);
                            stack.push(node_data.right);
                        } else {
                            // Do not modify stack if bounding box was not hit
                        }
                    }
                    BvhNode::Object(object_in) => {
                        match object_in.hit(ray_in, tmin, closest) {
                            Some(hit_record) => {
                                if hit_record.t < closest {
                                    closest = hit_record.t;
                                    closest_record = Some(hit_record);
                                }
                            }
                            None => {} // Continue searching
                        }
                    }
                }
            }

            closest_record
        };

        closest_record
    }
}

/// Constructs an axis-aligned bounding box from a Vec of hittable objects
fn bbox_from_objects(objects: &Vec<Hittable>) -> Aabb {
    if objects.len() == 1 {
        objects[0].get_bounding_box()
    } else {
        let mut result = Aabb::from_boxes(
            &objects[0].get_bounding_box(),
            &objects[1].get_bounding_box(),
        );
        for object in objects {
            result = Aabb::from_boxes(&result, &object.get_bounding_box());
        }
        result
    }
}

#[derive(Clone)]
pub enum Hittable {
    Sphere(Sphere),
    Quad(Quad),
}

impl Hittable {
    fn get_bounding_box(&self) -> Aabb {
        match self {
            Hittable::Sphere(sphere) => sphere.bounding_box.clone(),
            Hittable::Quad(quad) => quad.bounding_box.clone(),
        }
    }

    fn hit(&self, ray_in: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
        match self {
            Hittable::Sphere(sphere) => hit_sphere(ray_in, sphere, tmin, tmax),
            Hittable::Quad(quad) => hit_quad(ray_in, quad, tmin, tmax),
        }
    }
}
