use crate::{
    camera::{Camera, render},
    sphere::Sphere,
    vector::Vector3,
};

mod camera;
mod hit_record;
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

    // World geometries and materials
    let spheres = {
        vec![
            Sphere::new(
                Vector3 {
                    x: 0.0,
                    y: 0.0,
                    z: -1.0,
                },
                0.5,
            ),
            Sphere::new(
                Vector3 {
                    x: 0.0,
                    y: -100.5,
                    z: -1.0,
                },
                100.0,
            ),
        ]
    };

    // Render
    render(&mut camera, &spheres);
}
