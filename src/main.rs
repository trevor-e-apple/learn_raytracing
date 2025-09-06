use rand::{Rng, rngs::ThreadRng};

use crate::{
    camera::{Camera, render},
    material::Material,
    sphere::Sphere,
    vector::Vector3,
};

mod camera;
mod hit_record;
mod material;
mod math;
mod ray;
mod raytrace_vector;
mod sphere;
mod vector;

// We use a right-handed coordinate system

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 1200;
    let mut camera = Camera::new(
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
        50,
    );
    let max_depth = 50;

    // World geometries and materials
    let (materials, spheres) = {
        let mut world_rng = ThreadRng::default();
        let mut materials = vec![];
        let mut spheres = vec![];

        let material_ground = materials.len();
        materials.push(Material::Diffuse(Vector3 {
            x: 0.5,
            y: 0.5,
            z: 0.5,
        }));
        spheres.push(Sphere::new(Vector3 {x: 0.0, y: -1000.0, z: 0.0}, 1000.0, material_ground));

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
                    let sphere_material = if choose_mat < 0.8 {
                        // Diffuse
                        let albedo = Vector3 {
                            x: world_rng.random_range(0.0..1.0),
                            y: world_rng.random_range(0.0..1.0),
                            z: world_rng.random_range(0.0..1.0),
                        };
                        let sphere_material = materials.len();
                        materials.push(Material::Diffuse(albedo));

                        sphere_material
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

                        sphere_material
                    } else {
                        // Dielectric
                        let sphere_material = materials.len();
                        materials.push(Material::Dielectric(1.5));

                        sphere_material
                    };

                    spheres.push(Sphere::new(center, small_sphere_radius, sphere_material));
                }
            }
        }

        // Add some non-randomly placed spheres
        {
            let material1 = materials.len();
            materials.push(Material::Dielectric(1.5));
            spheres.push(Sphere::new(
                Vector3 {
                    x: 0.0,
                    y: 1.0,
                    z: 0.0,
                },
                1.0,
                material1,
            ));

            let material2 = materials.len();
            materials.push(Material::Diffuse(Vector3 {
                x: 0.4,
                y: 0.2,
                z: 0.1,
            }));
            spheres.push(Sphere::new(Vector3 {x: -4.0, y: 1.0, z: 0.0}, 1.0, material2));

            let material3 = materials.len();
            materials.push(Material::Metal(Vector3 { x: 0.7, y: 0.6, z: 0.5 }, 0.0));
            spheres.push(Sphere::new(Vector3 {x: 4.0, y: 1.0, z: 0.0}, 1.0, material3));
        }

        (materials, spheres)
    };

    // Render
    render(&mut camera, &spheres, &materials, max_depth);
}
