use crate::vector::Vector3;

#[derive(Clone, Default)]
pub struct Ray {
    pub origin: Vector3,    // The origin of the ray in world space
    pub direction: Vector3, // The direction that the ray points to (from the origin)
    pub time: f64,          // The time that the ray samples. Should be between 0.0 and 1.0
}

/// Linearly interpolate between ray origin and direction.
/// E.g. if t = 0.0, then the return value is the origin. If t = 1.0,
/// then the return value is origin + direction
pub fn at(ray: &Ray, t: f64) -> Vector3 {
    ray.origin + t * ray.direction
}
