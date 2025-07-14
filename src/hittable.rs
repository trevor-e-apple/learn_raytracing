use crate::{color::Vector3, ray::Ray};

pub struct HitRecord {
    pub point: Vector3,
    pub normal: Vector3,
    pub t: f64
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64, rec: &mut HitRecord) -> bool;
}