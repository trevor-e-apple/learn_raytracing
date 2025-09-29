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
        let u = {
            let u = p.x - p.x.floor();
            u * u * (3.0 - 2.0 * u)
        };
        let v = {
            let v = p.y - p.y.floor();
            v * v * (3.0 - 2.0 * v)
        };
        let w = {
            let w = p.z - p.z.floor();
            w * w * (3.0 - 2.0 * w)
        };

        let i = p.x.floor() as i64;
        let j = p.y.floor() as i64;
        let k = p.z.floor() as i64;

        let mut c: [[[f64; 2]; 2]; 2] = [[[0.0; 2]; 2]; 2];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.rand_float[self.perm_x
                        [((i + (di as i64)) as usize) & 255]
                        ^ self.perm_y[((j + (dj as i64)) as usize) & 255]
                        ^ self.perm_z[((k + (dk as i64)) as usize) & 255]];
                }
            }
        }

        // Perform trilinear interpolation
        {
            let mut accumulation = 0.0;
            for i in 0..2 {
                for j in 0..2 {
                    for k in 0..2 {
                        accumulation += ((i as f64 * u) + ((1 - i) as f64) * (1.0 - u))
                            * ((j as f64 * v) + ((1 - j) as f64) * (1.0 - v))
                            * ((k as f64 * w) + ((1 - k) as f64) * (1.0 - w))
                            * c[i][j][k];
                    }
                }
            }

            accumulation
        }

        // self.rand_float[self.perm_x[i as usize] ^ self.perm_y[j as usize] ^ self.perm_z[k as usize]]
    }

    fn generate_perm(rng: &mut ThreadRng) -> [usize; POINT_COUNT] {
        let mut p: [usize; POINT_COUNT] = [0; POINT_COUNT];

        for index in 0..p.len() {
            p[index] = index;
        }

        // Permute
        for index in (1..p.len()).rev() {
            let target = rng.random_range(0..index);
            let tmp = p[index];
            p[index] = p[target];
            p[target] = tmp;
        }

        p
    }
}
