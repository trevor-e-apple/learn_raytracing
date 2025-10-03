use crate::{
    aabb::Aabb,
    hit_record::HitRecord,
    ray::{self, Ray},
    vector::{Vector3, calc_cross_product},
};

/// Structure for flat parallelograms
#[derive(Clone)]
pub struct Quad {
    pub q: Vector3,
    pub u: Vector3,
    pub v: Vector3,
    pub normal: Vector3,
    pub material: usize,
    pub bounding_box: Aabb,
    w: Vector3,
    d: f64,
}

impl Quad {
    pub fn new(q: Vector3, u: Vector3, v: Vector3, material: usize) -> Self {
        let bounding_box = Aabb::from_boxes(&Aabb::new(q, q + u + v), &Aabb::new(q + u, q + v));
        let mut normal = calc_cross_product(&u, &v);
        normal.normalize();

        let d = Vector3::dot_product(&q, &normal);
        let w = Vector3::dot_product(&normal, &normal) * &normal;
        Self {
            q,
            u,
            v,
            d,
            w,
            normal,
            material,
            bounding_box,
        }
    }
}

pub fn hit_quad(ray_in: &Ray, quad_in: &Quad, tmin: f64, tmax: f64) -> Option<HitRecord> {
    let denom = Vector3::dot_product(&quad_in.normal, &ray_in.direction);

    // No hit if the ray is parallel to the plane
    if denom.abs() < 1e-8 {
        return None;
    }

    // Return false if the hit point parameter t is outside the interval
    let t = (quad_in.d - Vector3::dot_product(&quad_in.normal, &ray_in.origin));
    if (t > tmax) || (t < tmin) {
        return None;
    }

    let intersection = ray::at(ray_in, t);
    let planar_hitpt_vector = intersection - quad_in.q;
    let alpha = Vector3::dot_product(
        &quad_in.w,
        &calc_cross_product(&planar_hitpt_vector, &quad_in.v),
    );
    let beta = Vector3::dot_product(
        &quad_in.w,
        &calc_cross_product(&quad_in.u, &planar_hitpt_vector),
    );

    // Check if we are in the interior of the quad
    let (u, v) = {
        if (alpha < 0.0 || alpha > 1.0) || (beta < 0.0 || beta > 1.0) {
            return None;
        } else {
            (alpha, beta)
        }
    };

    Some(HitRecord::new(
        ray_in,
        quad_in.normal,
        t,
        quad_in.material,
        u,
        v,
    ))
}
