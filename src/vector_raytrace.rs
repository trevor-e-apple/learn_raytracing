// For raytracing-specific vector functions
use crate::color::Vector3;

pub fn near_zero(v: Vector3) -> bool {
    let s = 1e-8;
    v.x.abs() < s && v.y.abs() < s && v.z.abs() < s
}

/// Returns the vector corresponding to the reflection of v off of surface with normal n
pub fn reflect(v: &Vector3, n: &Vector3) -> Vector3 {
    *v - 2.0 * Vector3::dot_product(v, n) * (*n)
}

pub fn refract(uv: &Vector3, n: &Vector3, etai_over_etat: f64) -> Vector3 {
    let cos_theta = f64::min(Vector3::dot_product(&(-1.0 * *uv), n), 1.0);

    let r_out_perp = etai_over_etat * (uv + &(cos_theta * n));
    let r_out_parallel = -1.0 * (1.0 - r_out_perp.magnitude_squared()).abs().sqrt() * n;
    r_out_perp + r_out_parallel
}
