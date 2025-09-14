use std::rc::Rc;

use crate::{aabb::{hit_aabb, Aabb}, hit_record::HitRecord, ray::Ray, sphere::{hit_sphere, Sphere}, vector::Vector3};

#[derive(Clone)]
struct NodeData {
    left: Rc<BvhNode>,
    right: Rc<BvhNode>,
    bbox: Aabb,
}

enum BvhNode {
    Node(NodeData),
    Object(usize),
}

pub struct Hittables {
    objects: Vec<Sphere>,
    bbox: Aabb,
    bvh: Rc<BvhNode>
}

impl Hittables {
    pub fn new() -> Self {
        Self {
            objects: vec![],
            bbox: Aabb::new(
                Vector3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                Vector3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
            ),
            bvh: todo!("Construct me"),
        }
    }

    pub fn add_sphere(&mut self, s: Sphere) -> usize {
        self.bbox = Aabb::from_boxes(&self.bbox, &s.bounding_box);
        let handle = self.objects.len();
        self.objects.push(s);

        handle
    }
}


/// Use a bounding volume hierarchy to find the closest hit record 
pub fn get_hit_record(hittables: &Hittables, ray_in: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
    let mut stack: Vec<Rc<BvhNode>> = vec![hittables.bvh.clone()];

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
                    stack.push(node_data.left.clone());
                    stack.push(node_data.right.clone());
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
