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
    let image_width = 400;
    let mut camera = Camera::new(
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
        aspect_ratio,
        image_width,
        90.0,
        100,
    );
    let max_depth = 50;

    // World geometries and materials
    let (
        materials,
        material_ground,
        material_center,
        material_left,
        material_bubble,
        material_right,
    ) = {
        let mut materials = vec![];

        let material_ground = materials.len();
        materials.push(Material::Diffuse(Vector3 {
            x: 0.8,
            y: 0.8,
            z: 0.0,
        }));

        let material_center = materials.len();
        materials.push(Material::Diffuse(Vector3 {
            x: 0.1,
            y: 0.2,
            z: 0.5,
        }));

        let material_left = materials.len();
        materials.push(Material::Dielectric(1.5));

        let material_bubble = materials.len();
        materials.push(Material::Dielectric(1.0 / 1.5));

        let material_right = materials.len();
        materials.push(Material::Metal(
            Vector3 {
                x: 0.8,
                y: 0.6,
                z: 0.2,
            },
            1.0,
        ));

        (
            materials,
            material_ground,
            material_center,
            material_left,
            material_bubble,
            material_right,
        )
    };

    let spheres = {
        vec![
            Sphere::new(
                Vector3 {
                    x: 0.0,
                    y: -100.5,
                    z: -1.0,
                },
                100.0,
                material_ground,
            ),
            Sphere::new(
                Vector3 {
                    x: 0.0,
                    y: 0.0,
                    z: -1.2,
                },
                0.5,
                material_center,
            ),
            Sphere::new(
                Vector3 {
                    x: -1.0,
                    y: 0.0,
                    z: -1.0,
                },
                0.5,
                material_left,
            ),
            Sphere::new(
                Vector3 {
                    x: -1.0,
                    y: 0.0,
                    z: -1.0,
                },
                0.4,
                material_bubble,
            ),
            Sphere::new(
                Vector3 {
                    x: 1.0,
                    y: 0.0,
                    z: -1.0,
                },
                0.5,
                material_right,
            ),
        ]
    };

    // Render
    render(&mut camera, &spheres, &materials, max_depth);
}
