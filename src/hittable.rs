use std::rc::Rc;

use crate::{color::Vector3, ray::Ray};

#[derive(Default, Clone)]
pub struct HitRecord {
    pub point: Vector3,
    pub normal: Vector3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vector3) {
        // Sets the hit record normal vector
        // NOTE: the parameter `outward_normal` is assumed to have unit length.

        self.front_face = Vector3::dot_product(&r.direction, &outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -1.0 * outward_normal
        };
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64, rec: &mut HitRecord) -> bool;
}

pub struct HittableList {
    pub objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object)
    }

    pub fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord {
            ..Default::default()
        };
        let mut hit_anything = false;
        let mut closest_so_far = ray_tmax;

        for object in &self.objects {
            if object.hit(r, ray_tmin, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }

        hit_anything
    }
}
