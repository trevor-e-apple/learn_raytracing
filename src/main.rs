use std::rc::Rc;

use rand::Rng;

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
        20.0,
        aspect_ratio,
        image_width,
        80,
        50,
        Vector3 {
            x: 13.0,
            y: 2.0,
            z: 3.0,
        },
        Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        Vector3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        10.0,
        0.6,
    );

    // RNG
    let mut rng = rand::rng();

    // Materials
    let ground_material = Rc::new(Lambertian {
        albedo: Color {
            x: 0.5,
            y: 0.5,
            z: 0.5,
        },
    });

    // World
    let mut world = HittableList {
        objects: vec![Rc::new(Sphere {
            center: Vector3 {
                x: 0.0,
                y: -1000.0,
                z: 0.0,
            },
            radius: 1000.0,
            mat: ground_material,
        })],
    };

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.random_range(0.0..1.0);
            let center = Vector3 {
                x: a as f64 + 0.9 * rng.random_range(0.0..1.0),
                y: 0.2,
                z: b as f64 + 0.9 * rng.random_range(0.0..1.0),
            };

            if (center
                - Vector3 {
                    x: 4.0,
                    y: 0.2,
                    z: 0.0,
                })
            .magnitude()
                > 0.9
            {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Vector3::random_new_range(&mut rng, 0.0, 1.0);
                    world.add(Rc::new(Sphere {
                        center,
                        radius: 0.2,
                        mat: Rc::new(Lambertian { albedo }),
                    }));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Vector3::random_new_range(&mut rng, 0.5, 1.0);
                    let fuzz = rng.random_range(0.0..0.5);
                    world.add(Rc::new(Sphere {
                        center,
                        radius: 0.2,
                        mat: Rc::new(Metal { albedo, fuzz }),
                    }));
                } else {
                    // glass
                    world.add(Rc::new(Sphere {
                        center,
                        radius: 0.2,
                        mat: Rc::new(Dielectric {
                            refraction_index: 1.5,
                        }),
                    }))
                }
            }
        }
    }

    let material1 = Rc::new(Dielectric {
        refraction_index: 1.5,
    });
    world.add(Rc::new(Sphere {
        center: Vector3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        radius: 1.0,
        mat: material1,
    }));

    let material2 = Rc::new(Lambertian {
        albedo: Color {
            x: 0.4,
            y: 0.2,
            z: 0.1,
        },
    });
    world.add(Rc::new(Sphere {
        center: Vector3 {
            x: -4.0,
            y: 1.0,
            z: 0.0,
        },
        radius: 1.0,
        mat: material2,
    }));

    let material3 = Rc::new(Metal {
        albedo: Color {
            x: 0.7,
            y: 0.6,
            z: 0.5,
        },
        fuzz: 0.0,
    });
    world.add(Rc::new(Sphere {
        center: Vector3 {
            x: 4.0,
            y: 1.0,
            z: 0.0,
        },
        radius: 1.0,
        mat: material3,
    }));

    camera.render(&world);
}
