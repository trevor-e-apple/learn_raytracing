use crate::{
    color::{write_color, Color},
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
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: i32) -> Self {
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

        Self {
            aspect_ratio,
            image_width,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    pub fn render(&self, world: &HittableList) {
        // Output ppm file
        println!("P3");
        println!("{} {} 255", self.image_width, self.image_height);

        for j in 0..self.image_height {
            for i in 0..self.image_width {
                let pixel_center = self.pixel00_loc
                    + (i as f64) * self.pixel_delta_u
                    + (j as f64) * self.pixel_delta_v;
                let ray = Ray {
                    origin: self.center,
                    direction: pixel_center - self.center,
                };
                let pixel_color = self.ray_color(&ray, &world);

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
}
