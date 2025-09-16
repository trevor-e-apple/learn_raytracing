use std::rc::Rc;

use crate::{aabb::{hit_aabb, Aabb}, hit_record::HitRecord, ray::Ray, sphere::{hit_sphere, Sphere}, vector::Vector3};

#[derive(Clone)]
struct NodeData {
    left: Option<Rc<BvhNode>>,
    right: Option<Rc<BvhNode>>,
    bbox: Aabb,
}

enum BvhNode {
    Node(NodeData),
    Object(Sphere),
}

pub struct Hittables {
    objects: Vec<Sphere>,
    // bbox: Aabb,
    bvh: Option<Rc<BvhNode>>
}

impl Hittables {
    pub fn new() -> Self {
        Self {
            objects: vec![],
            // bbox: Aabb::new(
            //     Vector3 {
            //         x: 0.0,
            //         y: 0.0,
            //         z: 0.0,
            //     },
            //     Vector3 {
            //         x: 0.0,
            //         y: 0.0,
            //         z: 0.0,
            //     },
            // ),
            bvh: None,
        }
    }

    pub fn add_sphere(&mut self, s: Sphere) -> usize {
        // self.bbox = Aabb::from_boxes(&self.bbox, &s.bounding_box);
        let handle = self.objects.len();
        self.objects.push(s);

        // Set BVH to None since we need to reconstruct it now
        self.bvh = None;

        handle
    }

    /// Use a bounding volume hierarchy to find the closest hit record 
    pub fn get_hit_record(&mut self, ray_in: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
        // Get the bvh
        let bvh = match self.bvh {
            Some(bvh) => bvh.clone(),
            None => {
                // Construct BVH if it does not already exist

                // Initialize stack
                let all_contained_objects = {
                    let mut all_contained_objects: Vec<Sphere> = Vec::<Sphere>::with_capacity(self.objects.len());
                    for sphere in &self.objects {
                        all_contained_objects.push(sphere.clone());
                    }
                    all_contained_objects
                };
                let all_bbox = bbox_from_spheres(&all_contained_objects);
                let root = Rc::new(BvhNode::Node(NodeData { left: None, right: None, bbox: all_bbox }));
                let stack: Vec<(Rc<BvhNode>, Vec<Sphere>)> = vec![(root.clone(), all_contained_objects)];

                loop {
                    let (node, contained_objects) = match stack.pop() {
                        Some(contained_objects) => contained_objects,
                        None => break,
                    };

                    if contained_objects.len() == 1 {
                        let left = BvhNode::Object(contained_objects[0].clone());
                        // No need to put back on stack
                    } else if contained_objects.len() == 2 {
                        let left = BvhNode::Object(contained_objects[0].clone());
                        let right = BvhNode::Object(contained_objects[1].clone());
                        // No need to put back on stack
                    } else {
                        // Sort the objects by the longest axis
                        match bbox_from_spheres(&contained_objects).get_longest_axis() {
                            crate::aabb::Axis::X => contained_objects.sort_by(|a, b| a.bounding_box.get_center().x.total_cmp(&b.bounding_box.get_center().x)),
                            crate::aabb::Axis::Y => contained_objects.sort_by(|a, b| a.bounding_box.get_center().y.total_cmp(&b.bounding_box.get_center().y)),
                            crate::aabb::Axis::Z => contained_objects.sort_by(|a, b| a.bounding_box.get_center().z.total_cmp(&b.bounding_box.get_center().z)),
                        }

                        let (left_spheres, right_spheres) = contained_objects.split_at(contained_objects.len() / 2);
                        let left_spheres = left_spheres.to_vec();
                        let right_spheres = right_spheres.to_vec();
                        
                        // Put half of the objects in the left and half the objects in the right
                        let left = Rc::new(BvhNode::Node(NodeData { left: None, right: None, bbox: bbox_from_spheres(&left_spheres) }));
                        let right = Rc::new(BvhNode::Node(NodeData { left: None, right: None, bbox: bbox_from_spheres(&right_spheres) }));

                        // Push both nodes onto stack
                        stack.push((left, left_spheres));
                        stack.push((right, right_spheres));
                    }
                }

                // 
                
                // Assign bvh root to hittables
                self.bvh = Some(root.clone());

                root
            },
        };

        let mut stack: Vec<Rc<BvhNode>> = vec![bvh];

        let mut closest_record: Option<HitRecord> = None;
        let mut closest = tmax;

        loop {
            let current_node = match stack.pop() {
                Some(current) => current,
                None => break,
            };

            match current_node.as_ref() {
                BvhNode::Node(node_data) => {
                    if hit_aabb(&node_data.bbox, ray_in, tmin, tmax) {
                        // Add both left and right to the stack
                        stack.push(node_data.left.expect("Missing left node").clone());
                        stack.push(node_data.right.expect("Missing right node").clone());
                    }
                    else {
                        // Do not modify stack if bounding box was not hit
                    }
                },
                BvhNode::Object(handle) => {
                    let sphere_in = &hittables.objects[*handle];
                    match hit_sphere(ray_in, sphere_in, tmin, closest) {
                        Some(hit_record) => {
                            if hit_record.t < closest {
                                closest = hit_record.t;
                                closest_record = Some(hit_record);
                            }
                        },
                        None => {}, // Continue searching
                    }
                },
            }
        }

        closest_record
    }
}

/// Constructs an axis-aligned bounding box from a Vec of spheres
fn bbox_from_spheres(spheres: &Vec<Sphere>) -> Aabb {
    let mut result = Aabb::new(
        Vector3 { x: 0.0, y: 0.0, z: 0.0 },
        Vector3 { x: 0.0, y: 0.0, z: 0.0 }
    );
    for sphere in spheres {
        result = Aabb::from_boxes(&result, &sphere.bounding_box);
    }    

    result
}
