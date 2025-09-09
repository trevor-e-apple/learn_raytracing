use rand::{Rng, rngs::ThreadRng};

use crate::{
    ray::Ray,
    raytrace_vector::{random_vector, reflect, refract},
    vector::Vector3,
};

pub enum Material {
    Diffuse(Vector3),    // albedo
    Metal(Vector3, f64), // albedo, fuzz radius
    Dielectric(f64),     // The ratio of the enclosed media's eta to the enclosing media's eta
}

/// Scatter a ray off of a material.
/// If the ray was complete absorbed, the function returns None.
/// If the ray scatters, the function returns a Vector3 corresponding to the attenuation of the color and the scattered ray.s  
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
                time: ray_in.time,
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
                time: ray_in.time,
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
                // If we hit the front face, we need to switch the refraction index to have enclosing media's eta over the enclosed media's
                1.0 / *ri
            } else {
                *ri
            };

            let unit_direction = Vector3::calc_normalized_vector(&ray_in.direction);

            // Determine whether we need to reflect or refract
            let direction = {
                let cos_theta = f64::min(
                    Vector3::dot_product(&(-1.0 * unit_direction), &hit_point_normal),
                    1.0,
                );
                let sin_theta = (1.0 - (cos_theta * cos_theta)).sqrt();

                let cannot_refract = (refraction_index * sin_theta) > 1.0;

                if cannot_refract
                    || reflectance(cos_theta, refraction_index) > rng.random_range(0.0..1.0)
                {
                    reflect(&unit_direction, &hit_point_normal)
                } else {
                    refract(&unit_direction, &hit_point_normal, refraction_index)
                }
            };

            let scattered_ray = Ray {
                origin: hit_point,
                direction,
                time: ray_in.time,
            };
            Some((attenuation, scattered_ray))
        }
    }
}

/// Calculates the reflectance of a dielectric material using Schlick's approximation.
fn reflectance(cos_theta: f64, refraction_index: f64) -> f64 {
    let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
    let r0 = r0 * r0;

    r0 + (1.0 - r0) * (1.0 - cos_theta).powi(5)
}
