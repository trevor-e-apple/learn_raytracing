use rand::rngs::ThreadRng;

use crate::{random_vector::random_vector, ray::Ray, vector::Vector3};

pub enum Material {
    Diffuse(f64),
    Metal,
}

/// Scatter a ray off of a material
pub fn scatter_ray(
    hit_material: &Material,
    ray_in: &Ray,
    hit_point: Vector3,
    hit_point_normal: Vector3,
    rng: &mut ThreadRng,
) -> Option<(f64, Ray)> {
    match hit_material {
        Material::Diffuse(attenuation) => {
            let reflected_ray = Ray {
                origin: hit_point,
                direction: hit_point_normal + random_vector(rng),
            };

            Some((*attenuation, reflected_ray))
        }
        Material::Metal => {
            todo!()
        }
    }
}
