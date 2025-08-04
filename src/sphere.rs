use std::rc::Rc;

use crate::{
    color::Vector3,
    hittable::{HitRecord, Hittable},
    material::Material,
};

pub struct Sphere {
    pub center: Vector3,
    pub radius: f64,
    pub mat: Rc<dyn Material>,
}

impl Hittable for Sphere {
    fn hit(&self, r: &crate::ray::Ray, ray_tmin: f64, ray_tmax: f64, rec: &mut HitRecord) -> bool {
        let oc = self.center - r.origin;
        let a = r.direction.magnitude_squared();
        let h = Vector3::dot_product(&r.direction, &oc);
        let c = oc.magnitude_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range (quadratics can have multiple solutions!)
        let root = {
            let mut root = (h - sqrtd) / a;
            if (root <= ray_tmin) || (ray_tmax <= root) {
                root = (h + sqrtd) / a;
                if (root <= ray_tmin) || (ray_tmax <= root) {
                    return false;
                }
            }
            root
        };

        rec.t = root;
        rec.point = r.at(rec.t);
        let outward_normal = (1.0 / self.radius) * (rec.point - self.center);
        rec.set_face_normal(r, &outward_normal);
        rec.mat = Some(self.mat.clone());

        true
    }
}
