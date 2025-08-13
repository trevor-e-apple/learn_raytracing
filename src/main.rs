use crate::vector::Vector3;

mod ray;
mod vector;

// We use a right-handed coordinate system

fn main() {
    let aspect_ratio = 16.0 / 9.0;

    let image_width = 400;
    let image_height = {
        // Calculate the image height using the aspect ratio
        let image_height = (image_width as f64 / aspect_ratio) as i32;
        if image_height > 0 { image_height } else { 1 }
    };

    let viewport_height = 2.0;
    // We don't reuse the aspect_ratio for calculating the viewport_width b/c that is the idealized ratio (not the actual ratio)
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

    // Camera
    // I think the focal length is arbitrary
    let focal_length = 1.0;
    let camera_center = Vector3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    // Pixels are inset by half the pixel-to-pixel distance so that the viewport area is evenly divided into width x height regions
    let pixel_spacing_x = viewport_width / (image_width) as f64;
    let pixel_spacing_y = viewport_height / (image_height) as f64;
    let top_left_pixel = {
        // camera_center +  + Vector3 { x: todo!(), y: todo!(), z: todo!() }
        let viewport_upper_left = camera_center
            + Vector3 {
                x: -1.0 * viewport_width / 2.0,
                y: viewport_height / 2.0,
                z: -1.0 * focal_length,
            };
        viewport_upper_left
            + Vector3 {
                x: pixel_spacing_x / 2.0,
                y: pixel_spacing_y / 2.0,
                z: 0.0,
            }
    };

    // Render
    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for x in 0..image_width {
        for y in 0..image_height {
            let r: i32 = {
                let r = (x as f64) / (image_width as f64);
                (255.99 * r) as i32
            };
            let g: i32 = {
                let g = (y as f64) / (image_height as f64);
                (255.99 * g) as i32
            };
            let b: i32 = 0;

            println!("{} {} {}", r, g, b);
        }
    }
}
