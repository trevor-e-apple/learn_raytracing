use rand::{Rng, rngs::ThreadRng};

use crate::vector::Vector3;

const POINT_COUNT: usize = 256;

pub struct Perlin {
    rand_float: [f64; POINT_COUNT],
    perm_x: [usize; POINT_COUNT],
    perm_y: [usize; POINT_COUNT],
    perm_z: [usize; POINT_COUNT],
}

impl Perlin {
    pub fn new() -> Self {
        let mut rng = ThreadRng::default();
        let mut rand_float: [f64; POINT_COUNT] = [0.0; POINT_COUNT];
        for index in 0..POINT_COUNT {
            rand_float[index] = rng.random_range(0.0..1.0);
        }

        let perm_x = Self::generate_perm(&mut rng);
        let perm_y = Self::generate_perm(&mut rng);
        let perm_z = Self::generate_perm(&mut rng);

        Self {
            rand_float,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    pub fn noise(&self, p: &Vector3) -> f64 {
        let i = ((4.0 * p.x) as usize) & 255;
        let j = ((4.0 * p.y) as usize) & 255;
        let k = ((4.0 * p.z) as usize) & 255;

        self.rand_float[self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k]]
    }

    fn generate_perm(rng: &mut ThreadRng) -> [usize; POINT_COUNT] {
        let mut p: [usize; POINT_COUNT] = [0; POINT_COUNT];

        for index in 0..p.len() {
            p[index] = index;
        }

        // Permute
        for index in (0..p.len()).rev() {
            let target = rng.random_range(0..index);
            let tmp = p[index];
            p[index] = p[target];
            p[target] = tmp;
        }

        p
    }
}
