use crate::{aabb::Aabb, sphere::Sphere, vector::Vector3};

pub struct Hittables {
    objects: Vec<Sphere>,
    bbox: Aabb,
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
        }
    }

    pub fn add_sphere(&mut self, s: Sphere) -> usize {
        self.bbox = Aabb::from_boxes(&self.bbox, &s.bounding_box);
        let handle = self.objects.len();
        self.objects.push(s);

        handle
    }
}
