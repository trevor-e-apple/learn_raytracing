use rand::{Rng, rngs::ThreadRng};

use crate::{
    color::{Color, Vector3},
    hittable::HitRecord,
    ray::Ray,
    vector_raytrace::{near_zero, reflect, refract},
};

pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
        rng: &mut ThreadRng,
    ) -> bool;
}

#[derive(Clone)]
pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
        rng: &mut ThreadRng,
    ) -> bool {
        let scatter_direction = {
            let scatter_direction = rec.normal + Vector3::random_unit_vector(rng);

            // Catch degenerate scatter direction
            if near_zero(scatter_direction) {
                rec.normal
            } else {
                scatter_direction
            }
        };

        *scattered = Ray {
            origin: rec.point,
            direction: scatter_direction,
        };
        *attenuation = self.albedo;
        true
    }
}

#[derive(Clone)]
pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
        rng: &mut ThreadRng,
    ) -> bool {
        let reflected = reflect(&r_in.direction, &rec.normal);
        let reflected = Vector3::calc_normalized_vector(&reflected)
            + self.fuzz * Vector3::random_unit_vector(rng);
        *scattered = Ray {
            origin: rec.point,
            direction: reflected,
        };
        *attenuation = self.albedo;

        Vector3::dot_product(&scattered.direction, &rec.normal) > 0.0
    }
}

pub struct Dielectric {
    pub refraction_index: f64,
}

impl Dielectric {
    fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        // Schlick's approximation for reflectance
        let r0 = {
            let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
            r0 * r0
        };

        r0 + (1.0 - r0) * f64::powi(1.0 - cosine, 5)
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
        rng: &mut ThreadRng,
    ) -> bool {
        *attenuation = Color {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };
        let ri = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = Vector3::calc_normalized_vector(&r_in.direction);
        let cos_theta = f64::min(
            Vector3::dot_product(&(-1.0 * unit_direction), &rec.normal),
            1.0,
        );
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = ri * sin_theta > 1.0;

        let direction: Vector3 =
            if cannot_refract || Self::reflectance(cos_theta, ri) > rng.random_range(0.0..1.0) {
                reflect(&unit_direction, &rec.normal)
            } else {
                refract(&unit_direction, &rec.normal, ri)
            };

        *scattered = Ray {
            origin: rec.point,
            direction: direction,
        };

        true
    }
}
