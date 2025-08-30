use rand::rngs::ThreadRng;

use crate::{
    ray::Ray,
    raytrace_vector::{random_vector, reflect},
    vector::Vector3,
};

pub enum Material {
    Diffuse(Vector3),
    Metal(Vector3),
}

/// Scatter a ray off of a material
pub fn scatter_ray(
    hit_material: &Material,
    ray_in: &Ray,
    hit_point: Vector3,
    hit_point_normal: Vector3,
    rng: &mut ThreadRng,
) -> Option<(Vector3, Ray)> {
    match hit_material {
        Material::Diffuse(albedo) => {
            // We could either scatter with some probability, and if it doesn't scatter, it's absorbed
            // completely. Or we could do what we do here: always scatter and have a constant attenuation.

            // If the random vector is ~= -1.0 * hit_point_normal, this vector's magnitude
            // can be ~= 0.0. This can cause issues, so we treat that as if it were the normal
            let scattered_direction = hit_point_normal + random_vector(rng);
            let scattered_direction = if scattered_direction.magnitude() < 1e-8 {
                hit_point_normal
            } else {
                scattered_direction
            };

            let scattered_ray = Ray {
                origin: hit_point,
                direction: scattered_direction,
            };

            Some((*albedo, scattered_ray))
        }
        Material::Metal(albedo) => {
            // Note that the direction of the ray is in world space as well as the normal, so no 
            // transforms are needed here. 
            let scattered_direction = reflect(&ray_in.direction, &hit_point_normal);
            let scattered_ray = Ray {
                origin: hit_point,
                direction: scattered_direction,
            };
            Some((*albedo, scattered_ray))
        }
    }
}
