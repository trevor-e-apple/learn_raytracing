use rand::{Rng, rngs::ThreadRng};

use crate::vector::Vector3;

/// Returns a random unit vector
pub fn random_vector(rng: &mut ThreadRng) -> Vector3 {
    loop {
        let result = Vector3 {
            x: rng.random_range(-1.0..1.0),
            y: rng.random_range(-1.0..1.0),
            z: rng.random_range(-1.0..1.0),
        };

        let magnitude_squared = result.magnitude_squared();
        // We need to make sure we don't square too small of a vector
        if magnitude_squared <= 1.0 && magnitude_squared >= 1e-160 {
            return (1.0 / magnitude_squared.sqrt()) * result;
        }
    }
}

/// Returns a random unit vector that faces the same hemisphere as a surface normal
pub fn random_on_hemisphere(rng: &mut ThreadRng, normal: Vector3) -> Vector3 {
    let vector = random_vector(rng);

    if Vector3::dot_product(&vector, &normal) > 0.0 {
        vector
    } else {
        -1.0 * vector
    }
}

/// Reflects the vector v off of a surface defined by the normal vector n.
/// Assumes that n is a unit vector
pub fn reflect(v: &Vector3, n: &Vector3) -> Vector3 {
    let b = Vector3::dot_product(&(-1.0 * v), n);
    v + &(2.0 * b * n)
}

/// Refracts the vector uv on a surface defined by the normal vector n
/// according to Snell's law. The ratio of the indices of refraction is the
/// etai_over_etat term.
/// Assumes that both vectors are unit vectors.
pub fn refract(uv: &Vector3, n: &Vector3, etai_over_etat: f64) -> Vector3 {
    let cos_theta = f64::min(Vector3::dot_product(&(-1.0 * uv), n), 1.0);
    let r_out_perp = etai_over_etat * (uv + &(n * cos_theta));
    let r_out_parallel = -1.0 * (1.0 - r_out_perp.magnitude_squared()).abs().sqrt() * n;
    r_out_perp + r_out_parallel
}
