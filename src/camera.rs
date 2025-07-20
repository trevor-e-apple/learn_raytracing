use rand::{Rng, rngs::ThreadRng};

use crate::{
    color::{Color, write_color},
    hittable::{HitRecord, HittableList},
    ray::Ray,
    vector::Vector3,
};

pub struct Camera {
    aspect_ratio: f64,
    image_width: i32,
    image_height: i32,
    center: Vector3,
    pixel00_loc: Vector3,
    pixel_delta_u: Vector3,
    pixel_delta_v: Vector3,
    samples_per_pixel: i32,
    pixel_samples_scale: f64,
    rng: ThreadRng,
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: i32, samples_per_pixel: i32) -> Self {
        let image_height = {
            let height = (image_width as f64 / aspect_ratio) as i32;

            if height > 1 { height } else { 1 }
        };

        let focal_length = 1.0;
        // Viewport dimensions can be real valued
        let viewport_height = 2.0;
        // We don't reuse aspect ratio since the image_height is not real-valued and doesn't represent the *true* ratio
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
        let center = Vector3 {
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
        let viewport_upper_left = center
            - Vector3 {
                x: 0.0,
                y: 0.0,
                z: focal_length,
            }
            - 0.5 * viewport_u
            - 0.5 * viewport_v;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        let pixel_samples_scale = 1.0 / samples_per_pixel as f64;

        Self {
            aspect_ratio,
            image_width,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            samples_per_pixel,
            pixel_samples_scale,
            rng: rand::rng(),
        }
    }

    pub fn render(&mut self, world: &HittableList) {
        // Output ppm file
        println!("P3");
        println!("{} {} 255", self.image_width, self.image_height);

        for j in 0..self.image_height {
            for i in 0..self.image_width {
                // Perform sampling
                let mut pixel_color = Color {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                };
                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color = pixel_color + self.ray_color(&r, world);
                }

                pixel_color = self.pixel_samples_scale * pixel_color;
                write_color(&pixel_color);
            }
        }
    }

    fn ray_color(&self, r: &Ray, world: &HittableList) -> Color {
        let mut hit_record = HitRecord {
            ..Default::default()
        };
        if world.hit(r, 0.0, std::f64::INFINITY, &mut hit_record) {
            let v = hit_record.normal
                + Vector3 {
                    x: 1.0,
                    y: 1.0,
                    z: 1.0,
                };
            0.5 * v
        } else {
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
    }

    /// Construct a camera ray originating from the origin and directed at randomly sampled point
    /// around the pixel location i,j
    fn get_ray(&mut self, i: i32, j: i32) -> Ray {
        let offset = self.sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i as f64) + offset.x) * self.pixel_delta_u
            + ((j as f64) + offset.y) * self.pixel_delta_v;
        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;

        Ray {
            origin: ray_origin,
            direction: ray_direction,
        }
    }

    /// Returns the vector to a random point in the [-.5, -.5] - [.5, .5] unit square
    fn sample_square(&mut self) -> Vector3 {
        Vector3 {
            x: self.rng.random_range(-0.5..0.5),
            y: self.rng.random_range(-0.5..0.5),
            z: 0.0,
        }
    }
}
