extern crate image;

use image::GenericImageView;
use std::sync::Arc;
use texture::perlin::Perlin;
use texture::texture::Texture;
use vec3::Vec3;

/// A Texture representing a constant color
#[derive(Copy, Clone, Debug)]
pub struct ConstantTexture {
    color: Vec3,
}

impl ConstantTexture {
    pub fn new(color: Vec3) -> Self {
        ConstantTexture { color }
    }
}

impl Texture for ConstantTexture {
    fn value(&self, _u: f64, _v: f64, _hit_point: &Vec3) -> Vec3 {
        self.color
    }
}

/// A texture representing a checkerboard pattern,
/// alternating between two textures `even` and `odd`
#[derive(Clone)]
pub struct CheckerTexture {
    even: Arc<Texture>,
    odd: Arc<Texture>,
}

impl CheckerTexture {
    pub fn new(even: Arc<Texture>, odd: Arc<Texture>) -> Self {
        CheckerTexture { even, odd }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, hit_point: &Vec3) -> Vec3 {
        let sines: f64 = (10.0 * hit_point.x()).sin()
            * (10.0 * hit_point.y()).sin()
            * (10.0 * hit_point.z()).sin();
        return if sines < 0.0 {
            self.odd.value(u, v, hit_point)
        } else {
            self.even.value(u, v, hit_point)
        };
    }
}

/// A texture representing a randomized, noisy pattern
/// generated with Perlin Noise
#[derive(Copy, Clone)]
pub struct NoiseTexture {
    pub noise: Perlin,
    pub frequency: f64,
    pub octaves: u8,
}

impl NoiseTexture {
    /// Constructs a new NoiseTexture
    /// #### Arguments:
    /// - `frequency`: controls the frequency of the noise's variance
    /// - `octaves`: controls the number of octaves to use during noise generation
    pub fn new(frequency: f64, octaves: u8) -> Self {
        NoiseTexture {
            noise: Perlin::new(),
            frequency,
            octaves,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, hit_point: &Vec3) -> Vec3 {
        // NOTE: This currently results in a marble-like texture,
        // and there is not a way for consumers of this texture to
        // configure anything aside from the frequency
        let sine = (self.frequency * hit_point.x()
            + 5.0
                * self
                    .noise
                    .turbulance(*hit_point * self.frequency, self.octaves, 1.0)
                    .abs())
        .sin();
        Vec3::new(1.0, 1.0, 1.0) * 0.5 * (1.0 + sine)
    }
}

/// A texture representing a loaded image
#[derive(Clone)]
pub struct ImageTexture {
    data: Vec<u8>,
    dimensions: (u32, u32),
}

impl ImageTexture {
    pub fn new(image_path: &str) -> Self {
        let img = image::open(image_path).unwrap();
        ImageTexture {
            data: img.raw_pixels(),
            dimensions: img.dimensions(),
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _hit_point: &Vec3) -> Vec3 {
        let (num_x, num_y) = self.dimensions;
        let mut i: i32 = (u * num_x as f64) as i32;
        let mut j: i32 = ((1.0 - v) * num_y as f64 - 0.0001) as i32;
        if i < 0 {
            i = 0;
        } else if i > (num_x as i32 - 1) {
            i = num_x as i32 - 1;
        }
        if j < 0 {
            j = 0;
        } else if j > (num_y as i32 - 1) {
            j = num_y as i32 - 1;
        }

        let idx = (3 * i + 3 * num_x as i32 * j) as usize;
        let r = self.data[idx] as f64 / 255.0;
        let g = self.data[idx + 1] as f64 / 255.0;
        let b = self.data[idx + 2] as f64 / 255.0;

        Vec3::new(r, g, b)
    }
}
