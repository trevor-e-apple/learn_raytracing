use crate::{aabb::Aabb, hit_record, ray::Ray};

struct NodeData {
    left: Box<BvhNode>,
    right: Box<BvhNode>,
    bbox: Aabb,
}

enum BvhNode {
    Node(NodeData),
    Object(usize),
}

pub fn hit_bvh(bvh: Box<BvhNode>, ray_in: &Ray, tmin: f64, tmax: f64) -> Option<usize> {
    let mut stack: Vec<Box<BvhNode>> = vec![];
    stack.push(bvh);

    loop {
        let current = match stack.pop() {
            Some(current) => current,
            None => break,
        };

        match current {
            BvhNode::Node(node_data) => todo!(),
            BvhNode::Object(_) => todo!(),
        }
    }

    None
}