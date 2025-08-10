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
    let mut camera = Camera::new(
        90.0,
        aspect_ratio,
        image_width,
        100,
        50,
        Vector3 {
            x: -2.0,
            y: 2.0,
            z: 1.0,
        },
        Vector3 {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        },
        Vector3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
    );

    // Materials
    let material_ground = Rc::new(Lambertian {
        albedo: Color {
            x: 0.8,
            y: 0.8,
            z: 0.0,
        },
    });
    let material_center = Rc::new(Lambertian {
        albedo: Color {
            x: 0.1,
            y: 0.2,
            z: 0.5,
        },
    });
    let material_left = Rc::new(Dielectric {
        refraction_index: 1.5,
    });
    let material_bubble = Rc::new(Dielectric {
        refraction_index: 1.0 / 1.5,
    });
    let material_right = Rc::new(Metal {
        albedo: Color {
            x: 0.8,
            y: 0.6,
            z: 0.2,
        },
        fuzz: 1.0,
    });

    // World
    let world = HittableList {
        objects: vec![
            Rc::new(Sphere {
                center: Vector3 {
                    x: 0.0,
                    y: -100.5,
                    z: -1.0,
                },
                radius: 100.0,
                mat: material_ground,
            }),
            Rc::new(Sphere {
                center: Vector3 {
                    x: 0.0,
                    y: 0.0,
                    z: -1.2,
                },
                radius: 0.5,
                mat: material_center,
            }),
            Rc::new(Sphere {
                center: Vector3 {
                    x: -1.0,
                    y: 0.0,
                    z: -1.0,
                },
                radius: 0.5,
                mat: material_left,
            }),
            Rc::new(Sphere {
                center: Vector3 {
                    x: -1.0,
                    y: 0.0,
                    z: -1.0,
                },
                radius: 0.4,
                mat: material_bubble,
            }),
            Rc::new(Sphere {
                center: Vector3 {
                    x: 1.0,
                    y: 0.0,
                    z: -1.0,
                },
                radius: 0.5,
                mat: material_right,
            }),
        ],
    };

    camera.render(&world);
}
