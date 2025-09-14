use crate::{ray::Ray, vector::Vector3};

// Structure for axis-aligned bounding box
pub struct Aabb {
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    z0: f64,
    z1: f64,
}

impl Aabb {
    pub fn new(a: Vector3, b: Vector3) -> Self {
        let (x0, x1) = if a.x <= b.x { (a.x, b.x) } else { (b.x, a.x) };

        let (y0, y1) = if a.y <= b.y { (a.y, b.y) } else { (b.y, a.y) };

        let (z0, z1) = if a.z <= b.z { (a.z, b.z) } else { (b.z, a.z) };

        Self {
            x0,
            x1,
            y0,
            y1,
            z0,
            z1,
        }
    }

    pub fn from_boxes(a: &Self, b: &Self) -> Self {
        let x0 = if a.x0 <= b.x0 { a.x0 } else { b.x0 };
        let x1 = if a.x1 >= b.x1 { a.x1 } else { b.x1 };
        let y0 = if a.y0 <= b.y0 { a.y0 } else { b.y0 };
        let y1 = if a.y1 >= b.y1 { a.y1 } else { b.y1 };
        let z0 = if a.z0 <= b.z0 { a.z0 } else { b.z0 };
        let z1 = if a.z1 >= b.z1 { a.z1 } else { b.z1 };
        Self {
            x0,
            x1,
            y0,
            y1,
            z0,
            z1,
        }
    }
}

pub fn hit_aabb(bounding_box: &Aabb, r: &Ray, tmin: f64, tmax: f64) -> bool {
    let origin = r.origin;
    let direction = r.direction;

    let mut tmin = tmin;
    let mut tmax = tmax;

    for axis in 0..3 {
        let (axis_min, axis_max, origin_component, direction_component) = if axis == 0 {
            (bounding_box.x0, bounding_box.x1, origin.x, direction.x)
        } else if axis == 1 {
            (bounding_box.y0, bounding_box.y1, origin.y, direction.y)
        } else if axis == 2 {
            (bounding_box.z0, bounding_box.z1, origin.z, direction.z)
        } else {
            panic!("Unexpected axis value");
        };

        let adinv = 1.0 / direction_component;

        let t0 = (axis_min - origin_component) * adinv;
        let t1 = (axis_max - origin_component) * adinv;

        if t0 < t1 {
            if t0 > tmin {
                tmin = t0;
            }
            if t1 < tmax {
                tmax = t1;
            }
        } else {
            if t1 > tmin {
                tmin = t1;
            }
            if t0 < tmax {
                tmax = t0;
            }
        }

        if tmax <= tmin {
            return false;
        }
    }

    true
}
