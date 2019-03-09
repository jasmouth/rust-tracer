use na::Vector3 as Vec3;

use ray::Ray;

pub struct Camera {
    pub lower_left_corner: Vec3<f64>,
    pub horizontal: Vec3<f64>,
    pub vertical: Vec3<f64>,
    pub origin: Vec3<f64>,
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            lower_left_corner: Vec3::new(-2.0, -1.0, -1.0),
            horizontal: Vec3::new(4.0, 0.0, 0.0),
            vertical: Vec3::new(0.0, 2.0, 0.0),
            origin: Vec3::new(0.0, 0.0, 0.0),
        }
    }

    pub fn create_ray(&self, u: f64, v: f64) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.lower_left_corner + u * self.horizontal + v * self.vertical
                - self.origin,
        }
    }
}
