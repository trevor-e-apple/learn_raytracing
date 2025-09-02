use crate::{
    hit_record::HitRecord,
    ray::{self, Ray},
    vector::Vector3,
};

pub struct Sphere {
    center: Vector3,
    radius: f64,
    material: usize, // Handle to the material that was hit
}

impl Sphere {
    pub fn new(center: Vector3, radius: f64, material: usize) -> Self {
        assert!(radius > 0.0);
        Self {
            center,
            radius,
            material,
        }
    }
}

/// Determines whether the ray intersects the sphere centered at 'center' with radius 'radius' within [tmin, tmax].
///
/// Having a valid range for tmin and tmax allows us to avoid solutions behind the camera, avoid reflecting off the inside
/// of the surface we're scattering off of, and also avoid solutions that are farther out than our closest solution.
///
/// Returns None if there is no intersection or Some(HitRecord).
pub fn hit_sphere(ray_in: &Ray, sphere_in: &Sphere, tmin: f64, tmax: f64) -> Option<HitRecord> {
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
    return Some(HitRecord::new(ray_in, normal, t, sphere_in.material));
}
