mod ray;
mod vector;

fn main() {
    let image_width = 256;
    let image_height = 256;

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
