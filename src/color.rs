pub use crate::vector::Vector3;

pub use Vector3 as Color;

pub fn write_color(color: &Color) {
    let r = color.x;
    let g = color.y;
    let b = color.z;

    let ir = (255.999 * r) as i64;
    let ig = (255.999 * g) as i64;
    let ib = (255.999 * b) as i64;

    println!("{} {} {}", ir, ig, ib);
}