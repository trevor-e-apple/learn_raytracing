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
