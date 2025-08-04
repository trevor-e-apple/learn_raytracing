/// For raytracing-specific vector functions
use crate::color::Vector3;

pub fn near_zero(v: Vector3) -> bool {
    let s = 1e-8;
    v.x.abs() < s && v.y.abs() < s && v.z.abs() < s
}

/// Returns the vector corresponding to the reflection of v off of surface with normal n
pub fn reflect(v: &Vector3, n: &Vector3) -> Vector3 {
    *v - 2.0 * Vector3::dot_product(v, n) * (*n)
}
