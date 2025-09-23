use crate::{
    aabb::Aabb,
    hit_record::HitRecord,
    ray::{self, Ray},
    vector::Vector3,
};

#[derive(Clone)]
pub struct Sphere {
    pub center: Ray,
    pub radius: f64,
    pub material: usize, // Handle to the material that was hit
    pub bounding_box: Aabb,
}

impl Sphere {
    /// Create an unmoving sphere
    pub fn new(center: Vector3, radius: f64, material: usize) -> Self {
        assert!(radius > 0.0);
        let radius_vector = Vector3 {
            x: radius,
            y: radius,
            z: radius,
        };
        let bounding_box = Aabb::new(center - radius_vector, center + radius_vector);
        Self {
            center: Ray {
                origin: center,
                direction: Vector3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                time: 0.0,
            },
            radius,
            material,
            bounding_box,
        }
    }

    /// Create a sphere that moves from center1 -> center2 over the course of t=0.0 -> t=1.0
    pub fn new_moving(center1: Vector3, center2: Vector3, radius: f64, material: usize) -> Self {
        assert!(radius > 0.0);
        let radius_vector = Vector3 {
            x: radius,
            y: radius,
            z: radius,
        };
        let box1 = Aabb::new(center1 - radius_vector, center1 + radius_vector);
        let box2 = Aabb::new(center2 - radius_vector, center2 + radius_vector);

        let bounding_box = Aabb::from_boxes(&box1, &box2);
        Self {
            center: Ray {
                origin: center1,
                direction: center2 - center1,
                time: 0.0,
            },
            radius,
            material,
            bounding_box,
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
    let current_center = ray::at(&sphere_in.center, ray_in.time);
    let radius = sphere_in.radius;

    let center_minus_origin = current_center - ray_in.origin;
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
    let normal = (1.0 / radius) * (point - current_center);

    let (u, v) = {
        // Return u,v coordinates valued from [0, 1] based on
        // phi (angle around the y-axis) having values from [0, 2 * pi] and theta (angle and the x-axis) having values from [0, pi].
        let theta = (-1.0 * normal.y).acos();
        let phi = (-1.0 * normal.z).atan2(normal.x) + std::f64::consts::PI;

        let u = phi / (2.0 * std::f64::consts::PI);
        let v = theta / std::f64::consts::PI;

        (u, v)
    };

    return Some(HitRecord::new(ray_in, normal, t, sphere_in.material, u, v));
}
