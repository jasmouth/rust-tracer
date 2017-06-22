use na::Vector3 as Vec3;

#[derive(Copy, Clone)]
pub struct Ray {
	pub origin: Vec3<f64>,
	pub direction: Vec3<f64>
}

impl Ray {
	pub fn point_at_param(&self, t: f64) -> Vec3<f64> {
		self.origin + t * self.direction
	}
}
