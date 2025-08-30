use crate::vector::Vector3;

#[derive(Clone, Default)]
pub struct Ray {
    pub origin: Vector3,    // The origin of the ray in world space
    pub direction: Vector3, // The ray target in world space (not in the space where the ray's origin is (0, 0, 0))
}

/// Linearly interpolate between ray origin and direction
pub fn at(ray: &Ray, t: f64) -> Vector3 {
    ray.origin + t * ray.direction
}
