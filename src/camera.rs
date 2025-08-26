use crate::{
    hit_record::HitRecord,
    ray::Ray,
    sphere::{Sphere, hit_sphere},
    vector::Vector3,
};

pub struct Camera {
    image_width: i32,
    image_height: i32,
    top_left_pixel: Vector3,
    pixel_spacing_x: f64,
    pixel_spacing_y: f64,
    center: Vector3,
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: i32) -> Self {
        let image_height = {
            // Calculate the image height using the aspect ratio
            let image_height = (image_width as f64 / aspect_ratio) as i32;
            if image_height > 0 { image_height } else { 1 }
        };

        let viewport_height = 2.0;
        // We don't reuse the aspect_ratio for calculating the viewport_width b/c that is the idealized ratio (not the actual ratio)
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        // Camera data
        let (top_left_pixel, pixel_spacing_x, pixel_spacing_y, center) = {
            // I think the focal length is arbitrary
            let focal_length = 1.0;
            let center = Vector3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            };

            // Pixels are inset by half the pixel-to-pixel distance so that the viewport area is evenly divided into width x height regions
            let pixel_spacing_x = viewport_width / (image_width as f64);
            let pixel_spacing_y = viewport_height / (image_height as f64);
            let top_left_pixel = {
                let viewport_upper_left = center
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

            (top_left_pixel, pixel_spacing_x, pixel_spacing_y, center)
        };

        Self {
            image_width,
            image_height,
            top_left_pixel,
            pixel_spacing_x,
            pixel_spacing_y,
            center,
        }
    }
}

pub fn render(camera: &Camera, spheres: &Vec<Sphere>) {
    // ppm format preamble
    println!("P3");
    println!("{} {}", camera.image_width, camera.image_height);
    println!("255");

    for y in 0..camera.image_height {
        eprintln!("Scanlines remaining: {}", camera.image_height - y);
        for x in 0..camera.image_width {
            // Note that we subtract the y values because we are going from the top down
            let current_pixel = Vector3 {
                x: camera.top_left_pixel.x + (x as f64) * camera.pixel_spacing_x,
                y: camera.top_left_pixel.y - (y as f64) * camera.pixel_spacing_y,
                z: camera.top_left_pixel.z,
            };
            let ray = Ray {
                origin: camera.center,
                direction: current_pixel - camera.center,
            };
            let color = ray_color(&ray, spheres);
            write_color(&color);
        }
    }
}

/// Get the color of the scene for a ray
fn ray_color(ray_in: &Ray, spheres: &Vec<Sphere>) -> Vector3 {
    // Find the closest geometry that the ray hit
    let closest_record = {
        let mut closest_record: Option<HitRecord> = None;
        let mut closest = std::f64::INFINITY;
        for sphere_geometry in spheres {
            match hit_sphere(ray_in, sphere_geometry, 0.0, closest) {
                Some(record) => {
                    closest = record.t;
                    closest_record = Some(record);
                }
                None => {}
            }
        }

        closest_record
    };

    match closest_record {
        Some(closest_record) => {
            // Map the normal vector (component's values [-1, 1]) to the color space (valued [0, 1])
            0.5 * Vector3 {
                x: closest_record.normal.x + 1.0,
                y: closest_record.normal.y + 1.0,
                z: closest_record.normal.z + 1.0,
            }
        }
        None => {
            // Create a background color
            let unit_vector = Vector3::calc_normalized_vector(&ray_in.direction);

            let white = Vector3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            };
            let blue = Vector3 {
                x: 0.5,
                y: 0.7,
                z: 1.0,
            };

            let lerp_value = (unit_vector.y + 1.0) / 2.0; // Y value has a range of -1.0 to 1.0, and we map that to 0.0 to 1.0 
            let blended = (1.0 - lerp_value) * white + lerp_value * blue;

            blended
        }
    }
}

/// Write a color out to stdout for the ppm format
fn write_color(color: &Vector3) {
    let r = (color.x * 255.99) as i32;
    let g = (color.y * 255.99) as i32;
    let b = (color.z * 255.99) as i32;

    println!("{} {} {}", r, g, b);
}
