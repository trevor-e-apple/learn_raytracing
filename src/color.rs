pub use crate::vector::Vector3;

pub use Vector3 as Color;

fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        linear_component.sqrt()
    } else {
        0.0
    }
}

pub fn write_color(color: &Color) {
    let r = linear_to_gamma(color.x);
    let g = linear_to_gamma(color.y);
    let b = linear_to_gamma(color.z);

    let r = r.clamp(0.0, 0.999);
    let g = g.clamp(0.0, 0.999);
    let b = b.clamp(0.0, 0.999);

    let ir = (255.999 * r) as i64;
    let ig = (255.999 * g) as i64;
    let ib = (255.999 * b) as i64;

    println!("{} {} {}", ir, ig, ib);
}
