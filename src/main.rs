use crate::{
    color::{Color, write_color},
    ray::Ray,
    vector::Vector3,
};

mod color;
mod ray;
mod vector;

fn ray_color(r: &Ray) -> Color {
    Color {
        x: 0.00,
        y: 0.00,
        z: 0.00,
    }
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
            let ray = Ray { origin: camera_center, direction: pixel_center - camera_center };
            let pixel_color = ray_color(&ray);

            write_color(&pixel_color);
        }
    }
}
