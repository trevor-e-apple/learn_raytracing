use rand::{Rng, rngs::ThreadRng};

use crate::{
    hit_record::HitRecord,
    material::{Material, scatter_ray},
    math::degrees_to_radians,
    ray::Ray,
    sphere::{Sphere, hit_sphere},
    vector::{Vector3, calc_cross_product},
};

pub struct Camera {
    image_width: i32,  // The height of the image in pixels
    image_height: i32, // The width of the image in pixels
    vfov: f64,         // Vertical field of view
    top_left_pixel: Vector3,
    pixel_spacing_u: Vector3, // The vector to add to a pixel to get the next horizontal pixel
    pixel_spacing_v: Vector3, // The vector to add to a pixel to get the next vertical pixel
    center: Vector3,          // The camera's center
    look_at: Vector3,
    vup: Vector3,
    pixel_sample_count: i32,
    one_over_pixel_sample_count: f64,

    rng: ThreadRng,
}

impl Camera {
    pub fn new(
        center: Vector3,
        look_at: Vector3,
        vup: Vector3,
        aspect_ratio: f64,
        image_width: i32,
        vfov: f64,
        pixel_sample_count: i32,
    ) -> Self {
        let image_height = {
            // Calculate the image height using the aspect ratio
            let image_height = (image_width as f64 / aspect_ratio) as i32;
            if image_height > 0 { image_height } else { 1 }
        };

        // Calculate the height of the viewport
        let focal_length = (center - look_at).magnitude();
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();

        // h is the opposite, focal length is the adjacent, hence the multiplication by focal length.
        // The multiplication by 2.0 is because we want the height of the whole viewport, not half of it.
        let viewport_height = 2.0 * h * focal_length;
        // We don't reuse the aspect_ratio for calculating the viewport_width b/c that is the idealized ratio (not the actual ratio)
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        // Calculate the vectors for our camera's coordinate system
        let (u, v, w) = {
            // Our coordinate system is right-handed, so vector w goes from the viewport towards the center
            let w = Vector3::calc_normalized_vector(&(center - look_at));
            let u = Vector3::calc_normalized_vector(&calc_cross_product(&vup, &w));
            let v = calc_cross_product(&w, &u);

            (u, v, w)
        };

        let viewport_u = viewport_width * u;
        let viewport_v = -1.0 * viewport_height * v;

        // Pixels are inset by half the pixel-to-pixel distance so that the viewport area is evenly divided into width x height regions
        let pixel_spacing_u = (1.0 / (image_width as f64)) * viewport_u;
        let pixel_spacing_v = (1.0 / (image_height as f64)) * viewport_v; // Negative since we go "down" the viewport

        let top_left_pixel = {
            // Term 1 is subtracted because w points away from the viewport
            // Term 2 is subtracted because we want the "leftmost" column, and u points to the "right"
            // Term 3 is subtracted because we want the "top" row, and v points "down"
            let viewport_upper_left =
                center - (focal_length * w) - (0.5 * viewport_u) - (0.5 * viewport_v);

            // We need to inset the top-left pixel
            viewport_upper_left + 0.5 * pixel_spacing_u + 0.5 * pixel_spacing_v
        };

        let camera_rng = ThreadRng::default();

        Self {
            image_width,
            image_height,
            vfov,
            top_left_pixel,
            pixel_spacing_u,
            pixel_spacing_v,
            center,
            look_at,
            vup,
            pixel_sample_count,
            one_over_pixel_sample_count: 1.0 / (pixel_sample_count as f64),
            rng: camera_rng,
        }
    }
}

/// Render the scene in the ppm format
///
/// camera: The camera data structure
/// spheres: The world geometry
/// materials: A reference to the materials data
/// max_depth: The maximum number of reflections for each ray
pub fn render(
    camera: &mut Camera,
    spheres: &Vec<Sphere>,
    materials: &Vec<Material>,
    max_depth: i32,
) {
    // ppm format preamble
    println!("P3");
    println!("{} {}", camera.image_width, camera.image_height);
    println!("255");

    for y in 0..camera.image_height {
        eprintln!("Scanlines remaining: {}", camera.image_height - y);
        for x in 0..camera.image_width {
            // Note that we subtract the y values because we are going from the top down
            let current_pixel = camera.top_left_pixel
                + (x as f64) * camera.pixel_spacing_u
                + (y as f64) * camera.pixel_spacing_v;

            // find the color of the current pixel
            let pixel_color = {
                let mut average_color = Vector3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                };
                for _ in 0..camera.pixel_sample_count {
                    // Pick a random point in the unit square around the current pixel
                    let sample_pixel = current_pixel
                        + camera.rng.random_range(-0.5..0.5) * camera.pixel_spacing_u
                        + camera.rng.random_range(-0.5..0.5) * camera.pixel_spacing_v;
                    let ray = Ray {
                        origin: camera.center,
                        direction: sample_pixel - camera.center,
                    };
                    average_color = average_color
                        + ray_color(&ray, spheres, &mut camera.rng, materials, max_depth);
                }

                average_color = camera.one_over_pixel_sample_count * average_color;

                average_color
            };

            write_color(&pixel_color);
        }
    }
}

/// Get the color of the scene for a ray
///
/// ray_in: The ray to determine the reflection of
/// spheres: The world geometries
/// rng: An RNG for generating randomness in our reflections
/// materials: A reference to the materials data
/// max_depth: The maximum number of remaining reflections to calculate
fn ray_color(
    ray_in: &Ray,
    spheres: &Vec<Sphere>,
    rng: &mut ThreadRng,
    materials: &Vec<Material>,
    max_depth: i32,
) -> Vector3 {
    if max_depth <= 0 {
        return Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
    }

    // Find the closest geometry that the ray hit
    let closest_record = {
        let mut closest_record: Option<HitRecord> = None;
        let mut closest = std::f64::INFINITY;
        for sphere_geometry in spheres {
            // Due to floating-point imprecision, occasionally the intersection point is not
            // exactly flush with the surface of the geometry. This can cause a ray to reflect
            // off of the surface that it is reflecting off of. We set tmin to some small value
            // greater than 0.0 to avoid this.
            match hit_sphere(ray_in, sphere_geometry, 0.001, closest) {
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
            let material = &materials[closest_record.material];
            match scatter_ray(
                material,
                ray_in,
                closest_record.point,
                closest_record.normal,
                closest_record.front_face,
                rng,
            ) {
                Some((attenuation, reflected_ray)) => {
                    // Recursively look up color of the reflected ray
                    let recursive_result =
                        ray_color(&reflected_ray, spheres, rng, materials, max_depth - 1);
                    Vector3 {
                        x: recursive_result.x * attenuation.x,
                        y: recursive_result.y * attenuation.y,
                        z: recursive_result.z * attenuation.z,
                    }
                }
                None => {
                    // Ray was absorbed
                    Vector3 {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                    }
                }
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
    // Gamma correct color first
    let gamma_corrected_r = color.x.sqrt();
    let gamma_corrected_g = color.y.sqrt();
    let gamma_corrected_b = color.z.sqrt();

    let r = (gamma_corrected_r * 255.99) as i32;
    let g = (gamma_corrected_g * 255.99) as i32;
    let b = (gamma_corrected_b * 255.99) as i32;

    println!("{} {} {}", r, g, b);
}
