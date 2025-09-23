use std::{env, rc::Rc};

use rand::{Rng, rngs::ThreadRng};

use crate::{
    camera::{Camera, render},
    hittables::Hittables,
    map::{CheckerData, ImageData},
    material::Material,
    sphere::Sphere,
    vector::Vector3,
};

mod aabb;
mod camera;
mod hit_record;
mod hittables;
mod map;
mod material;
mod math;
mod ray;
mod raytrace_vector;
mod sphere;
mod vector;

// We use a right-handed coordinate system

fn bouncing_spheres() -> (Camera, Vec<Material>, Hittables, i32) {
    // Initialize camera
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let camera = Camera::new(
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
        0.6,
        10.0,
        aspect_ratio,
        image_width,
        20.0,
        100,
    );
    let max_depth = 50;

    let mut world_rng = ThreadRng::default();
    let mut materials: Vec<Material> = vec![];
    let mut hittables = Hittables::new();

    let material_ground = materials.len();
    materials.push(Material::Diffuse(map::Map::Checker(CheckerData::new(
        0.32,
        Rc::new(map::Map::Color(Vector3 {
            x: 0.2,
            y: 0.3,
            z: 0.1,
        })),
        Rc::new(map::Map::Color(Vector3 {
            x: 0.9,
            y: 0.9,
            z: 0.9,
        })),
    ))));
    hittables.add_sphere(Sphere::new(
        Vector3 {
            x: 0.0,
            y: -1000.0,
            z: 0.0,
        },
        1000.0,
        material_ground,
    ));

    // Make a bunch of small spheres with different materials
    let small_sphere_radius = 0.2;
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = world_rng.random_range(0.0..1.0);
            let center = Vector3 {
                x: (a as f64) + 0.9 * world_rng.random_range(0.0..1.0),
                y: 0.2,
                z: (b as f64) + 0.9 * world_rng.random_range(0.0..1.0),
            };

            if (center
                - Vector3 {
                    x: 0.4,
                    y: 0.2,
                    z: 0.0,
                })
            .magnitude()
                > 0.9
            {
                if choose_mat < 0.8 {
                    // Diffuse
                    let albedo = Vector3 {
                        x: world_rng.random_range(0.0..1.0),
                        y: world_rng.random_range(0.0..1.0),
                        z: world_rng.random_range(0.0..1.0),
                    };
                    let sphere_material = materials.len();
                    materials.push(Material::Diffuse(map::Map::Color(albedo)));

                    // These spheres are falling
                    hittables.add_sphere(Sphere::new_moving(
                        center,
                        center
                            + Vector3 {
                                x: 0.0,
                                y: world_rng.random_range(0.0..0.5),
                                z: 0.0,
                            },
                        small_sphere_radius,
                        sphere_material,
                    ));
                } else if choose_mat < 0.95 {
                    // Metal
                    let albedo = Vector3 {
                        x: world_rng.random_range(0.5..1.0),
                        y: world_rng.random_range(0.5..1.0),
                        z: world_rng.random_range(0.5..1.0),
                    };
                    let fuzz = world_rng.random_range(0.0..0.5);
                    let sphere_material = materials.len();
                    materials.push(Material::Metal(albedo, fuzz));

                    hittables.add_sphere(Sphere::new(center, small_sphere_radius, sphere_material));
                } else {
                    // Dielectric
                    let sphere_material = materials.len();
                    materials.push(Material::Dielectric(1.5));

                    hittables.add_sphere(Sphere::new(center, small_sphere_radius, sphere_material));
                }
            }
        }
    }

    // Add some non-randomly placed spheres
    {
        let material1 = materials.len();
        materials.push(Material::Dielectric(1.5));
        hittables.add_sphere(Sphere::new(
            Vector3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
            1.0,
            material1,
        ));

        let material2 = materials.len();
        materials.push(Material::Diffuse(map::Map::Color(Vector3 {
            x: 0.4,
            y: 0.2,
            z: 0.1,
        })));
        hittables.add_sphere(Sphere::new(
            Vector3 {
                x: -4.0,
                y: 1.0,
                z: 0.0,
            },
            1.0,
            material2,
        ));

        let material3 = materials.len();
        materials.push(Material::Metal(
            Vector3 {
                x: 0.7,
                y: 0.6,
                z: 0.5,
            },
            0.0,
        ));
        hittables.add_sphere(Sphere::new(
            Vector3 {
                x: 4.0,
                y: 1.0,
                z: 0.0,
            },
            1.0,
            material3,
        ));
    }

    (camera, materials, hittables, max_depth)
}

fn checkered_spheres() -> (Camera, Vec<Material>, Hittables, i32) {
    // Initialize camera
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let camera = Camera::new(
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
        0.0,
        10.0,
        aspect_ratio,
        image_width,
        20.0,
        100,
    );
    let max_depth = 50;

    // Initialize world
    let mut materials: Vec<Material> = vec![];
    let mut hittables = Hittables::new();

    let checker = materials.len();
    materials.push(Material::Diffuse(map::Map::Checker(CheckerData::new(
        0.32,
        Rc::new(map::Map::Color(Vector3 {
            x: 0.2,
            y: 0.3,
            z: 0.1,
        })),
        Rc::new(map::Map::Color(Vector3 {
            x: 0.9,
            y: 0.9,
            z: 0.9,
        })),
    ))));

    hittables.add_sphere(Sphere::new(
        Vector3 {
            x: 0.0,
            y: -10.0,
            z: 0.0,
        },
        10.0,
        checker,
    ));
    hittables.add_sphere(Sphere::new(
        Vector3 {
            x: 0.0,
            y: 10.0,
            z: 0.0,
        },
        10.0,
        checker,
    ));

    (camera, materials, hittables, max_depth)
}

fn globe(file_path: &str) -> (Camera, Vec<Material>, Hittables, i32) {
    // Initialize camera
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let camera = Camera::new(
        Vector3 {
            x: 0.0,
            y: 0.0,
            z: 12.0,
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
        0.0,
        10.0,
        aspect_ratio,
        image_width,
        20.0,
        100,
    );
    let max_depth = 50;

    // Initialize world
    let mut materials: Vec<Material> = vec![];
    let mut hittables = Hittables::new();

    let earth_texture = materials.len();
    materials.push(Material::Diffuse(map::Map::Image(ImageData::new(
        file_path,
    ))));
    hittables.add_sphere(Sphere::new(
        Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        2.0,
        earth_texture,
    ));

    (camera, materials, hittables, max_depth)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let scene: i32 = if args.len() == 1 {
        0
    } else {
        args[1].parse().expect("Unable to parse scene arg")
    };

    // Generate scene
    let (mut camera, materials, mut hittables, max_depth) = if scene == 0 {
        bouncing_spheres()
    } else if scene == 1 {
        checkered_spheres()
    } else {
        let earth_image_path = &args[2];
        globe(earth_image_path)
    };

    // Render
    render(&mut camera, &mut hittables, &materials, max_depth);
}
