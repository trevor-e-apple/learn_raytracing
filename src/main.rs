use crate::color::{write_color, Color};

mod color;
mod vector;
mod ray;

fn main() {
    let image_width = 256;
    let image_height = 256;

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
