use vec3::Vec3;

/// Defines a simple Ray
#[derive(Copy, Clone, Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
    /// The inverse of the ray's direction (i.e. 1 / direction.z())
    pub invert_direction: Vec3,
    /// Used to determine whether the components of invert_direction are negative
    pub sign: [bool; 3],
    pub time: f64,
}

impl Ray {
    /// Constructs a new Ray
    pub fn new(origin: Vec3, direction: Vec3, time: f64) -> Self {
        let invert_direction = Vec3::new(
            1.0 / direction.x(),
            1.0 / direction.y(),
            1.0 / direction.z(),
        );
        let sign = [
            invert_direction.x() < 0.0,
            invert_direction.y() < 0.0,
            invert_direction.z() < 0.0,
        ];
        Ray {
            origin,
            direction,
            invert_direction,
            sign,
            time,
        }
    }

    pub fn point_at_param(&self, t: f64) -> Vec3 {
        self.origin + t * self.direction
    }
}
