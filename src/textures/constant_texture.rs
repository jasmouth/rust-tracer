use textures::texture::Texture;
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
