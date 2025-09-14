use crate::{
    ray::{self, Ray},
    vector::Vector3,
};

pub struct HitRecord {
    pub point: Vector3, // The point of intersection

    // The normal at the point of intersection. Normals always point out and are always unit vectors.
    pub normal: Vector3,

    pub t: f64,           // The t value for the ray at the point of intersection
    pub front_face: bool, // Whether or not the ray intersected from the front face or the back face

    pub material: usize, // Handle to the material that was hit
}

impl HitRecord {
    /// Constructor for the hit record. 'normal' is assumed to have unit length.
    pub fn new(ray_in: &Ray, normal: Vector3, t: f64, material: usize) -> Self {
        let point = ray::at(&ray_in, t);
        let front_face = Vector3::dot_product(&ray_in.direction, &normal) < 0.0;
        let normal = if front_face { normal } else { -1.0 * normal };

        Self {
            point,
            normal,
            t,
            front_face,
            material,
        }
    }
}
