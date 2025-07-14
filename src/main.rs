use crate::{
    color::{Color, write_color},
    ray::Ray,
    vector::Vector3,
};

mod color;
mod ray;
mod vector;

fn hit_sphere(center: &Vector3, radius: f64, ray: &Ray) -> bool {
    let oc = center - &ray.origin;
    let a = Vector3::dot_product(&ray.direction, &ray.direction);
    let b = -2.0 * Vector3::dot_product(&ray.direction, &oc);
    let c = Vector3::dot_product(&oc, &oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    discriminant >= 0.0
}

fn ray_color(r: &Ray) -> Color {
    if hit_sphere(&Vector3 { x: 0.0, y: 0.0, z: -1.0 }, 0.5, r) {
        return Color{x: 1.0, y: 0.0, z: 0.0};
    }

    let unit_direction = Vector3::calc_normalized_vector(&r.direction);
    let a = 0.5 * (unit_direction.y + 1.0);

    let white_level = (1.0 - a)
        * Color {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };
    let blue_level = a * Color {
        x: 0.5,
        y: 0.7,
        z: 1.0,
    };

    white_level + blue_level
}

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = {
        let height = (image_width as f64 / aspect_ratio) as i32;

        if height > 1 { height } else { 1 }
    };

    // Camera
    let focal_length = 1.0;
    // Viewport dimensions can be real valued
    let viewport_height = 2.0;
    // We don't reuse aspect ratio since the image_height is not real-valued and doesn't represent the *true* ratio
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Vector3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    // Calculate the vectors across the viewport
    let viewport_u = Vector3 {
        x: viewport_width,
        y: 0.0,
        z: 0.0,
    };
    let viewport_v = Vector3 {
        x: 0.0,
        y: -1.0 * viewport_height,
        z: 0.0,
    };

    // Calculate the horizontal and vertical delta vectors from pixel to pixel
    let pixel_delta_u = (1.0 / image_width as f64) * viewport_u;
    let pixel_delta_v = (1.0 / image_height as f64) * viewport_v;

    // Calculate the location of the upper left pixel
    let viewport_upper_left = camera_center
        - Vector3 {
            x: 0.0,
            y: 0.0,
            z: focal_length,
        }
        - 0.5 * viewport_u
        - 0.5 * viewport_v;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    // Output ppm file
    println!("P3");
    println!("{} {} 255", image_width, image_height);

    for j in 0..image_height {
        for i in 0..image_width {
            let pixel_center =
                pixel00_loc + (i as f64) * pixel_delta_u + (j as f64) * pixel_delta_v;
            let ray = Ray {
                origin: camera_center,
                direction: pixel_center - camera_center,
            };
            let pixel_color = ray_color(&ray);

            write_color(&pixel_color);
        }
    }
}
