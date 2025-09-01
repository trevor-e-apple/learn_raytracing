use rand::rngs::ThreadRng;

use crate::{
    ray::Ray,
    raytrace_vector::{random_vector, reflect, refract},
    vector::Vector3,
};

pub enum Material {
    Diffuse(Vector3), // albedo
    Metal(Vector3, f64), // albedo, fuzz radius
    Dielectric(f64), // index of refraction relative to air 
}

/// Scatter a ray off of a material
pub fn scatter_ray(
    hit_material: &Material,
    ray_in: &Ray,
    hit_point: Vector3,
    hit_point_normal: Vector3,
    front_face: bool,
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
        Material::Metal(albedo, fuzz) => {
            let reflected = {
                let mut reflected = reflect(&ray_in.direction, &hit_point_normal);

                // Normalize the reflected ray's vector in order to have consistent magnitude
                reflected.normalize();
                // Add fuzz
                reflected + (*fuzz * random_vector(rng))
            };

            let scattered_ray = Ray {
                origin: hit_point,
                direction: reflected,
            };
            Some((*albedo, scattered_ray))
        }
        Material::Dielectric(ri) => {
            let attenuation = Vector3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            };

            let refraction_index = if front_face {
                // If we hit the front face, we assume that we are hitting the glass from the air
                1.0 / *ri
            } else {
                *ri
            };

            let unit_direction = Vector3::calc_normalized_vector(&ray_in.direction);

            // Determine whether we need to reflect or refract
            let direction = {
                let cos_theta = f64::min(Vector3::dot_product(&(-1.0 * unit_direction), &hit_point_normal), 1.0);
                let sin_theta = (1.0 - (cos_theta * cos_theta)).sqrt();

                let cannot_refract = (refraction_index * sin_theta) > 1.0;

                if cannot_refract {
                    reflect(&unit_direction, &hit_point_normal)
                } else {
                    refract(&unit_direction, &hit_point_normal, refraction_index)
                }
            };

            let scattered_ray = Ray {
                origin: hit_point,
                direction,
            };
            Some((attenuation, scattered_ray))
        }
    }
}
