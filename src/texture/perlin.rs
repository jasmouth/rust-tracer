use rand::Rng;
use vec3::{dot, unit_vector, Vec3};

/// Represents a Perlin Noise generator
#[derive(Copy, Clone)]
pub struct Perlin {
    rand_vec: [Vec3; 256],
    x_perm: [u32; 256],
    y_perm: [u32; 256],
    z_perm: [u32; 256],
}

impl Perlin {
    /// Constructs a new Perlin Noise generator
    pub fn new() -> Self {
        let mut rand_vec: [Vec3; 256] = [Vec3::new(0.0, 0.0, 0.0); 256];
        let mut rng = rand::thread_rng();

        for i in 0..256 {
            rand_vec[i] = unit_vector(Vec3::new(
                -1.0 + 2.0 * rng.gen::<f64>(),
                -1.0 + 2.0 * rng.gen::<f64>(),
                -1.0 + 2.0 * rng.gen::<f64>(),
            ));
        }

        Perlin {
            rand_vec,
            x_perm: gen_perm(),
            y_perm: gen_perm(),
            z_perm: gen_perm(),
        }
    }

    /// Calculates the noise value at the given `hit_point` of a Ray
    pub fn noise(&self, hit_point: &Vec3) -> f64 {
        let (x, y, z) = (hit_point.x(), hit_point.y(), hit_point.z());
        let (x_flr, y_flr, z_flr) = (x.floor(), y.floor(), z.floor());

        let (u, v, w) = (x - x_flr, y - y_flr, z - z_flr);
        let (i, j, k) = (x_flr as i32, y_flr as i32, z_flr as i32);
        let mut c: [[[Vec3; 2]; 2]; 2] = [[[Vec3::new(0.0, 0.0, 0.0); 2]; 2]; 2];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let scrambled = self.x_perm[((i + di) as u8) as usize]
                        ^ self.y_perm[((j + dj) as u8) as usize]
                        ^ self.z_perm[((k + dk) as u8) as usize];
                    c[di as usize][dj as usize][dk as usize] = self.rand_vec[scrambled as usize];
                }
            }
        }
        perlin_interpolate(c, u, v, w)
    }

    /// Calculates a composite noise value by summing multiple noise
    /// values, up to a given `depth`
    /// #### Arguments
    /// - `hit_point`: The point at which to generate the noise value
    /// - `depth`: The number of noise values that will be generated
    /// - `weight`: The initial weight to be given to the noise values.
    ///   - Note: The weight is halved for each subsequent noise value
    ///   generated after the first. The weight is also used to jitter
    ///   the hit_point, so a higher weight corresponds to a less unified
    ///   noise pattern
    pub fn turbulance(&self, hit_point: Vec3, depth: u8, mut weight: f64) -> f64 {
        let mut acc = 0.0;
        for _ in 0..depth {
            acc += weight * self.noise(&(hit_point / weight));
            weight *= 0.5;
        }
        acc
    }
}

/// Generates a randomly shuffled array of the discrete values 0-255
fn gen_perm() -> [u32; 256] {
    let mut perm: [u32; 256] = [0; 256];
    let mut rng = rand::thread_rng();
    for i in 0..256 {
        perm[i] = i as u32;
    }
    for i in 255usize..0 {
        let target = (rng.gen::<f64>() * (i + 1) as f64) as usize;
        let temp = perm[i];
        perm[i] = perm[target];
        perm[target] = temp;
    }
    perm
}

/// Performs trilinear interpolation for smoothing the noise
fn perlin_interpolate(c: [[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
    let uu = u * u * (3.0 - 2.0 * u);
    let vv = v * v * (3.0 - 2.0 * v);
    let ww = w * w * (3.0 - 2.0 * w);
    let mut acc = 0.0;
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let weight = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
                acc += (i as f64 * uu + (1 - i) as f64 * (1.0 - uu))
                    * (j as f64 * vv + (1 - j) as f64 * (1.0 - vv))
                    * (k as f64 * ww + (1 - k) as f64 * (1.0 - ww))
                    * dot(&c[i][j][k], &weight);
            }
        }
    }

    acc
}
