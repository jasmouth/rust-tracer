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

    fn box_clone(&self) -> Box<Texture> {
        Box::new((*self).clone())
    }
}

/// A texture representing a checkerboard pattern,
/// alternating between two textures `even` and `odd`
#[derive(Clone)]
pub struct CheckerTexture {
    even: Box<Texture>,
    odd: Box<Texture>,
}

impl CheckerTexture {
    pub fn new(even: Box<Texture>, odd: Box<Texture>) -> Self {
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

    fn box_clone(&self) -> Box<Texture> {
        Box::new((*self).clone())
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
        let sine = (self.frequency * hit_point.z()
            + 10.0 * self.noise.turbulance(&hit_point, self.octaves))
        .sin();
        Vec3::new(1.0, 1.0, 1.0) * 0.5 * (1.0 + sine)
    }

    fn box_clone(&self) -> Box<Texture> {
        Box::new((*self).clone())
    }
}
