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
mod random_vector;
mod ray;
mod sphere;
mod vector;

// We use a right-handed coordinate system

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let mut camera = Camera::new(aspect_ratio, image_width, 100);
    let max_depth = 50;

    // World geometries and materials
    let materials = vec![
        Material::Diffuse(0.5), // Gray diffuse material
    ];
    let spheres = {
        vec![
            Sphere::new(
                Vector3 {
                    x: 0.0,
                    y: 0.0,
                    z: -1.0,
                },
                0.5,
                0,
            ),
            Sphere::new(
                Vector3 {
                    x: 0.0,
                    y: -100.5,
                    z: -1.0,
                },
                100.0,
                0,
            ),
        ]
    };

    // Render
    render(&mut camera, &spheres, &materials, max_depth);
}
