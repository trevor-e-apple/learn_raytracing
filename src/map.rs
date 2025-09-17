use std::rc::Rc;

use crate::vector::Vector3;

pub enum Map {
    Color(Vector3),
    Checker(CheckerData),
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
    }
}
