use crate::color::{write_color, Color};

mod color;
mod vector;
mod ray;

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = {
        let height = (image_width as f64 / aspect_ratio) as i32;

        if height > 1 {
            height
        } else {
            1
        }
    };

    // Viewport dimensions can be real valued
    let viewport_height = 2.0;
    // We don't reuse aspect ratio since the image_height is not real-valued and doesn't represent the *true* ratio
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

    println!("P3");
    println!("{} {} 255", image_width, image_height);

    for j in 0..image_height {
        for i in 0..image_width {
            let pixel_color = Color { 
                x: (i as f64) / (image_width as f64 - 1.0),
                y: (j as f64) / (image_height as f64 - 1.0),
                z: 0.0
            };

            write_color(&pixel_color);
        }
    }
}
