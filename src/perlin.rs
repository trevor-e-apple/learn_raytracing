use rand::{Rng, rngs::ThreadRng};

use crate::{raytrace_vector::random_vector, vector::Vector3};

const POINT_COUNT: usize = 256;

pub struct Perlin {
    rand_vec: [Vector3; POINT_COUNT],
    perm_x: [usize; POINT_COUNT],
    perm_y: [usize; POINT_COUNT],
    perm_z: [usize; POINT_COUNT],
}

impl Perlin {
    pub fn new() -> Self {
        let mut rng = ThreadRng::default();
        let mut rand_vec: [Vector3; POINT_COUNT] = [Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }; POINT_COUNT];
        for index in 0..POINT_COUNT {
            rand_vec[index] = random_vector(&mut rng);
        }

        let perm_x = Self::generate_perm(&mut rng);
        let perm_y = Self::generate_perm(&mut rng);
        let perm_z = Self::generate_perm(&mut rng);

        Self {
            rand_vec,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    /// The noise function can return values from [-1, +1]
    pub fn noise(&self, p: &Vector3) -> f64 {
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();

        let i = p.x.floor() as i64;
        let j = p.y.floor() as i64;
        let k = p.z.floor() as i64;

        let mut c: [[[Vector3; 2]; 2]; 2] = [[[Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }; 2]; 2]; 2];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.rand_vec[self.perm_x[((i + (di as i64)) as usize) & 255]
                        ^ self.perm_y[((j + (dj as i64)) as usize) & 255]
                        ^ self.perm_z[((k + (dk as i64)) as usize) & 255]];
                }
            }
        }

        // Perform interpolation
        {
            let uu = u * u * (3.0 - 2.0 * u);
            let vv = v * v * (3.0 - 2.0 * v);
            let ww = w * w * (3.0 - 2.0 * w);
            let mut accumulation = 0.0;

            for i in 0..2 {
                for j in 0..2 {
                    for k in 0..2 {
                        let weight_v = Vector3 {
                            x: u - i as f64,
                            y: v - j as f64,
                            z: w - k as f64,
                        };
                        accumulation += ((i as f64 * uu) + ((1 - i) as f64) * (1.0 - uu))
                            * ((j as f64 * vv) + ((1 - j) as f64) * (1.0 - vv))
                            * ((k as f64 * ww) + ((1 - k) as f64) * (1.0 - ww))
                            * Vector3::dot_product(&c[i][j][k], &weight_v);
                    }
                }
            }

            accumulation
        }
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
