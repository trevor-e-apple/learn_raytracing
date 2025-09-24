use std::{fs, rc::Rc};

use zune_jpeg::{ImageInfo, JpegDecoder};

use crate::{perlin::Perlin, vector::Vector3};

pub enum Map {
    Color(Vector3),
    Checker(CheckerData),
    Image(ImageData),
    Noise(Perlin),
}

pub struct CheckerData {
    inv_scale: f64,
    even: Rc<Map>,
    odd: Rc<Map>,
}

impl CheckerData {
    pub fn new(scale: f64, even: Rc<Map>, odd: Rc<Map>) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            even,
            odd,
        }
    }
}

pub struct ImageData {
    pixels: Vec<u8>,
    image_info: ImageInfo,
}

impl ImageData {
    pub fn new(file_path: &str) -> Self {
        let file_contents =
            fs::read(file_path).expect(&format!("Unable to read file path at {}", file_path));
        let mut decoder = JpegDecoder::new(&file_contents);
        let pixels = decoder
            .decode()
            .expect(&format!("Unable to decode file contents of {}", file_path));
        let info = decoder
            .info()
            .expect("Unable to get image info after decoding");

        Self {
            pixels,
            image_info: info,
        }
    }
}

pub fn get_map_value(map: &Map, u: f64, v: f64, p: Vector3) -> Vector3 {
    match map {
        Map::Color(color) => *color,
        Map::Checker(checker_data) => {
            let x_int = (checker_data.inv_scale * p.x).floor() as i64;
            let y_int = (checker_data.inv_scale * p.y).floor() as i64;
            let z_int = (checker_data.inv_scale * p.z).floor() as i64;
            let is_even = ((x_int + y_int + z_int) % 2) == 0;
            if is_even {
                get_map_value(&checker_data.even, u, v, p)
            } else {
                get_map_value(&checker_data.odd, u, v, p)
            }
        }
        Map::Image(image_data) => {
            let pixels = &image_data.pixels;
            let info = &image_data.image_info;

            if info.height <= 0 {
                // Debugging aid if info is invalid
                return Vector3 {
                    x: 1.0,
                    y: 0.0,
                    z: 1.0,
                };
            }

            // Clamp input texture coordinates to [0, 1]
            let u = if u < 0.0 {
                0.0
            } else if u > 1.0 {
                1.0
            } else {
                u
            };
            let v = {
                let v = if v < 0.0 {
                    0.0
                } else if v > 1.0 {
                    1.0
                } else {
                    v
                };

                // Flip v to image coordinates
                1.0 - v
            };

            let i = (u * info.width as f64) as usize;
            let j = (v * info.height as f64) as usize;

            let r_index = (3 * j) * (info.width as usize) + (3 * i);
            let pixel_r = *pixels
                .get(r_index)
                .expect(&format!("Failure to get pixel at ({}, {})", i, j));
            let pixel_g = *pixels
                .get(r_index + 1)
                .expect(&format!("Failure to get pixel at ({}, {})", i, j));
            let pixel_b = *pixels
                .get(r_index + 2)
                .expect(&format!("Failure to get pixel at ({}, {})", i, j));

            let color_scale = 1.0 / 255.0; // For normalizing RGB values to [0.0, 1.0]

            Vector3 {
                x: color_scale * (pixel_r as f64),
                y: color_scale * (pixel_g as f64),
                z: color_scale * (pixel_b as f64),
            }
        }
        Map::Noise(noise) => {
            noise.noise(&p)
                * Vector3 {
                    x: 1.0,
                    y: 1.0,
                    z: 1.0,
                }
        }
    }
}
