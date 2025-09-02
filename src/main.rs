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
    let mut camera = Camera::new(aspect_ratio, image_width, 90.0, 100);
    let max_depth = 50;

    // World geometries and materials
    let (
        materials,
        material_left,
        material_right,
    ) = {
        let mut materials = vec![];

        let material_left = materials.len();
        materials.push(Material::Diffuse(Vector3 { x: 0.0, y: 0.0, z: 1.0 }));

        let material_right = materials.len();
        materials.push(Material::Diffuse(Vector3 { x: 1.0, y: 0.0, z: 0.0 }));

        (
            materials,
            material_left,
            material_right,
        )
    };

    let r = (std::f64::consts::PI / 4.0).cos();
    let spheres = {
        vec![
            Sphere::new(
                Vector3 {
                    x: -r,
                    y: 0.0,
                    z: -1.0,
                },
                r,
                material_left,
            ),
            Sphere::new(
                Vector3 {
                    x: r,
                    y: 0.0,
                    z: -1.0,
                },
                r,
                material_right,
            ),
        ]
    };

    // Render
    render(&mut camera, &spheres, &materials, max_depth);
}
