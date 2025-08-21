use crate::{ray::{self, Ray}, vector::Vector3};

pub struct HitRecord {
    point: Vector3, // The point of intersection
    normal: Vector3, // The normal at the point of intersection. Normals always point out
    t: f64, // The t value for the ray at the point of intersection
    front_face: bool, // Whether or not the ray intersected from the front face or the back face
}

impl HitRecord {
    pub fn new(ray: Ray, normal: Vector3, t: f64) -> Self {
        let point = ray::at(&ray, t);
        let front_face = Vector3::dot_product(&ray.direction, &normal) < 0.0;
        Self {
            point,
            normal,
            t,
            front_face
        }
    }
}
