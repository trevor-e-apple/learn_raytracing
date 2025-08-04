use rand::rngs::ThreadRng;

use crate::{
    color::{Color, Vector3},
    hittable::HitRecord,
    ray::Ray,
    vector_raytrace::{near_zero, reflect},
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
        *scattered = Ray {
            origin: rec.point,
            direction: reflected,
        };
        *attenuation = self.albedo;
        true
    }
}
