use crate::vector::Vector3;

#[derive(Clone, Default)]
pub struct Ray {
    pub origin: Vector3,    // The origin of the ray in world space
    pub direction: Vector3, // The direction that the ray points in (from the origin)
}

/// Linearly interpolate between ray origin and direction
pub fn at(ray: &Ray, t: f64) -> Vector3 {
    ray.origin + t * ray.direction
}
