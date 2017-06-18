use na::Vector3;

#[derive(Copy, Clone)]
pub struct Ray {
	pub origin: Vector3<f64>,
	pub direction: Vector3<f64>
}

impl Ray {
	pub fn point_at_param(&self, t: f64) -> Vector3<f64> {
		self.origin + t * self.direction
	}
}
