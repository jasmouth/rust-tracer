use textures::texture::Texture;
use vec3::Vec3;

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
