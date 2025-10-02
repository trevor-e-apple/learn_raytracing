use crate::{
    aabb::Aabb,
    hit_record::HitRecord,
    ray::Ray,
    vector::{self, Vector3},
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
    d: f64,
}

impl Quad {
    pub fn new(q: Vector3, u: Vector3, v: Vector3, material: usize) -> Self {
        let bounding_box = Aabb::from_boxes(&Aabb::new(q, q + u + v), &Aabb::new(q + u, q + v));
        let mut normal = vector::calc_cross_product(&u, &v);
        normal.normalize();

        let d = Vector3::dot_product(&q, &normal);
        Self {
            q,
            u,
            v,
            d,
            normal,
            material,
            bounding_box,
        }
    }
}

pub fn hit_quad(ray_in: &Ray, quad_in: &Quad, tmin: f64, tmax: f64) -> Option<HitRecord> {
    todo!()
}
