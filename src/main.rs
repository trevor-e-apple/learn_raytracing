use std::rc::Rc;

use crate::{
    camera::Camera,
    color::Color,
    hittable::HittableList,
    material::{Dielectric, Lambertian, Metal},
    sphere::Sphere,
    vector::Vector3,
};

mod camera;
mod color;
mod hittable;
mod material;
mod math;
mod random_vector;
mod ray;
mod sphere;
mod vector;
mod vector_raytrace;

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;

    // Camera
    let mut camera = Camera::new(90.0, aspect_ratio, image_width, 100, 50);

    let r = f64::cos(3.14 / 4.0);
    // Materials
    let material_left = Rc::new(Lambertian {
        albedo: Color {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        },
    });
    let material_right = Rc::new(Lambertian {
        albedo: Color {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        },
    });

    // World
    let world = HittableList {
        objects: vec![
            Rc::new(Sphere {
                center: Vector3 {
                    x: -1.0 * r,
                    y: 0.0,
                    z: -1.0,
                },
                radius: r,
                mat: material_left,
            }),
            Rc::new(Sphere {
                center: Vector3 {
                    x: r,
                    y: 0.0,
                    z: -1.0,
                },
                radius: r,
                mat: material_right,
            }),
        ],
    };

    camera.render(&world);
}
