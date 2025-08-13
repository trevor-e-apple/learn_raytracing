use crate::vector::Vector3;

#[derive(Clone, Default)]
pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3,
}

impl Ray {
    /// Linearly interpolate between ray origin and direction
    pub fn at(ray: &Self, t: f64) -> Vector3 {
        ray.origin + t * ray.direction
    } 
}