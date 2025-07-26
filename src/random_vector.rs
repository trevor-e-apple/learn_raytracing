use rand::{Rng, rngs::ThreadRng};

use crate::vector::Vector3;

impl Vector3 {
    pub fn random_new(rng: &mut ThreadRng) -> Self {
        Self {
            x: rng.random(),
            y: rng.random(),
            z: rng.random(),
        }
    }

    pub fn random_new_range(rng: &mut ThreadRng, min: f64, max: f64) -> Self {
        Self {
            x: rng.random_range(min..max),
            y: rng.random_range(min..max),
            z: rng.random_range(min..max),
        }
    }

    pub fn random_unit_vector(rng: &mut ThreadRng) -> Self {
        loop {
            let p = Self::random_new_range(rng, -1.0, 1.0);
            let mag_squared = p.magnitude_squared();
            if 1e-160 < mag_squared &&  mag_squared <= 1.0 {
                return (1.0 / mag_squared.sqrt()) * p;
            }
        }
    }

    pub fn random_on_hemisphere(rng: &mut ThreadRng, normal: &Self) -> Self {
        let on_unit_sphere = Self::random_unit_vector(rng);
        if Vector3::dot_product(&on_unit_sphere, normal) > 0.0 {
            // In same hemisphere as the surface normal
            on_unit_sphere
        } else {
            -1.0 * on_unit_sphere
        }
    }
}
