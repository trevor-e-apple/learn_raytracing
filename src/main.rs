use std::rc::Rc;

use crate::{camera::Camera, hittable::HittableList, sphere::Sphere, vector::Vector3};

mod camera;
mod color;
mod hittable;
mod math;
mod random_vector;
mod ray;
mod sphere;
mod vector;

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;

    // Camera
    let mut camera = Camera::new(aspect_ratio, image_width, 100, 50);

    // World
    let world = HittableList {
        objects: vec![
            Rc::new(Sphere {
                center: Vector3 {
                    x: 0.0,
                    y: 0.0,
                    z: -1.0,
                },
                radius: 0.5,
            }),
            Rc::new(Sphere {
                center: Vector3 {
                    x: 0.0,
                    y: -100.5,
                    z: -1.0,
                },
                radius: 100.0,
            }),
        ],
    };

    camera.render(&world);
}
