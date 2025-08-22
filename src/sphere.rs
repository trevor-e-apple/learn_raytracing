use crate::{
    hit_record::HitRecord,
    ray::{self, Ray},
    vector::Vector3,
};

pub struct Sphere {
    center: Vector3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Vector3, radius: f64) -> Self {
        assert!(radius > 0.0);
        Self { center, radius }
    }
}

/// Determines whether the ray intersects the sphere centered at 'center' with radius 'radius' within [tmin, tmax].
/// Returns None if there is no intersection or Some(HitRecord).
pub fn hit_sphere(ray_in: &Ray, sphere_in: &Sphere, tmin: f64, tmax: f64) -> Option<HitRecord> {
    // TODO: please explain the value of tmin and tmax
    let center = sphere_in.center;
    let radius = sphere_in.radius;

    let center_minus_origin = center - ray_in.origin;
    let a = ray_in.direction.magnitude_squared();
    let h = Vector3::dot_product(&ray_in.direction, &center_minus_origin);
    let c = Vector3::dot_product(&center_minus_origin, &center_minus_origin) - (radius * radius);

    let discriminant = h * h - a * c;
    if discriminant < 0.0 {
        return None;
    }

    // Find the closest root within the valid range
    let discriminant_sqrt = discriminant.sqrt();
    let t = {
        // By computing the subtraction first, we don't need to compare the two solutions to determine which one is closer.
        // If the subtraction is a valid solution, then it is the closest solution.
        let t = (h - discriminant_sqrt) / a;

        if t <= tmin || t >= tmax {
            // If t is not in the range, then try the other solution to the
            let t = (h + discriminant_sqrt) / a;
            if t <= tmin || t >= tmax {
                return None;
            }

            t
        } else {
            t
        }
    };

    let point = ray::at(&ray_in, t);
    let normal = (1.0 / radius) * (point - center);
    return Some(HitRecord::new(ray_in, normal, t));
}
