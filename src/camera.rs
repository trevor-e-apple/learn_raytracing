use rand::{Rng, rngs::ThreadRng};

use crate::{
    color::{Color, write_color},
    hittable::{HitRecord, HittableList},
    math::degrees_to_radians,
    ray::Ray,
    vector::{Vector3, calc_cross_product},
};

pub struct Camera {
    aspect_ratio: f64,
    image_width: i32,
    image_height: i32,
    vfov: f64,
    center: Vector3,
    pixel00_loc: Vector3,
    pixel_delta_u: Vector3,
    pixel_delta_v: Vector3,
    samples_per_pixel: i32,
    pixel_samples_scale: f64,
    rng: ThreadRng,
    max_depth: i32,
    look_from: Vector3,
    look_at: Vector3,
    vup: Vector3,       // Camera relative "up"
    u: Vector3,         // Camera frame basis vector
    v: Vector3,         // Camera frame basis vector
    w: Vector3,         // Camera frame basis vector
    defocus_angle: f64, // Variation angle of rays through each pixel
    focus_dist: f64,    // Distance from camera lookfrom point to plane of perfect focus
    defocus_disk_u: Vector3,
    defocus_disk_v: Vector3,
}

impl Camera {
    pub fn new(
        vfov: f64,
        aspect_ratio: f64,
        image_width: i32,
        samples_per_pixel: i32,
        max_depth: i32,
        look_from: Vector3,
        look_at: Vector3,
        vup: Vector3,
        focus_dist: f64,
        defocus_angle: f64,
    ) -> Self {
        let image_height = {
            let height = (image_width as f64 / aspect_ratio) as i32;

            if height > 1 { height } else { 1 }
        };

        let center = look_from;

        // Determine viewport dimensions
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focus_dist;
        // We don't reuse aspect ratio since the image_height is not real-valued and doesn't represent the *true* ratio
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        // Calculate the basis vectors for the camera space
        let w = Vector3::calc_normalized_vector(&(look_from - look_at));
        let u = Vector3::calc_normalized_vector(&calc_cross_product(&vup, &w));
        let v = calc_cross_product(&w, &u);

        // Calculate the vectors across the viewport
        let viewport_u = viewport_width * u;
        let viewport_v = -1.0 * viewport_height * v;

        // Calculate the horizontal and vertical delta vectors from pixel to pixel
        let pixel_delta_u = (1.0 / image_width as f64) * viewport_u;
        let pixel_delta_v = (1.0 / image_height as f64) * viewport_v;

        // Calculate the location of the upper left pixel
        let viewport_upper_left = center - focus_dist * w - 0.5 * viewport_u - 0.5 * viewport_v;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        let pixel_samples_scale = 1.0 / samples_per_pixel as f64;

        // Calculate the camera defocus disk basis vectors
        let defocus_radius = focus_dist * (degrees_to_radians(defocus_angle / 2.0)).tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Self {
            aspect_ratio,
            image_width,
            image_height,
            vfov,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            samples_per_pixel,
            pixel_samples_scale,
            rng: rand::rng(),
            max_depth: max_depth,
            look_from,
            look_at,
            vup,
            u,
            v,
            w,
            defocus_angle,
            focus_dist,
            defocus_disk_u,
            defocus_disk_v,
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
                    pixel_color = pixel_color + self.ray_color(&r, self.max_depth, world);
                }

                pixel_color = self.pixel_samples_scale * pixel_color;
                write_color(&pixel_color);
            }
        }
    }

    fn ray_color(&mut self, r: &Ray, depth: i32, world: &HittableList) -> Color {
        // If we've exceeded the ray bounce limit, no more light is gathered
        if depth <= 0 {
            return Color {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            };
        }

        let mut hit_record = HitRecord {
            point: Vector3 {
                ..Default::default()
            },
            normal: Vector3 {
                ..Default::default()
            },
            mat: None,
            t: 0.0,
            front_face: false,
        };

        // We ignore hits that are very close to the origin of the ray, since
        // -- that could be the result of floating-point rounding errors
        if world.hit(r, 0.001, std::f64::INFINITY, &mut hit_record) {
            let material = hit_record
                .mat
                .as_ref()
                .expect("Missing material in hit_record")
                .clone();
            let mut attenuation = Color {
                ..Default::default()
            };
            let mut scattered = Ray {
                origin: Vector3 {
                    ..Default::default()
                },
                direction: Vector3 {
                    ..Default::default()
                },
            };
            if material.scatter(
                r,
                &hit_record,
                &mut attenuation,
                &mut scattered,
                &mut self.rng,
            ) {
                let scattered_color = self.ray_color(&scattered, depth - 1, world);
                Color {
                    x: attenuation.x * scattered_color.x,
                    y: attenuation.y * scattered_color.y,
                    z: attenuation.z * scattered_color.z,
                }
            } else {
                Color {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                }
            }
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

    /// Construct a camera ray originating from the defocus disk and directed at randomly sampled point
    /// around the pixel location i,j
    fn get_ray(&mut self, i: i32, j: i32) -> Ray {
        let offset = self.sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i as f64) + offset.x) * self.pixel_delta_u
            + ((j as f64) + offset.y) * self.pixel_delta_v;
        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            let p = Vector3::random_in_unit_disk(&mut self.rng);
            self.center + (p.x * self.defocus_disk_u) + (p.y * self.defocus_disk_v)
        };
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
