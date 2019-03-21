use ray::Ray;
use vec3::Vec3;

/// Represents an Axis-aligned Bounding Box.
#[derive(Copy, Clone, Debug)]
pub struct AxisAlignedBoundingBox {
    pub min_bound: Vec3,
    pub max_bound: Vec3,
}

impl AxisAlignedBoundingBox {
    /// Creates a new AxisAlignedBoundingBox with the given min_bound and max_bound
    pub fn new(min_bound: Vec3, max_bound: Vec3) -> Self {
        AxisAlignedBoundingBox {
            min_bound,
            max_bound,
        }
    }

    /// Determines whether the given ray intersects this bounding box
    pub fn hit(&self, ray: &Ray, mut t_min: f64, mut t_max: f64) -> bool {
        for i in 0..3 {
            let invert_dir = 1.0 / ray.direction[i];
            let mut t_0 = (self.min_bound[i] - ray.origin[i]) / ray.direction[i];
            let mut t_1 = (self.max_bound[i] - ray.origin[i]) / ray.direction[i];
            if invert_dir < 0.0 {
                std::mem::swap(&mut t_0, &mut t_1);
            }
            if t_0 > t_min {
                t_min = t_0;
            }
            if t_1 < t_max {
                t_max = t_1;
            }
            if t_max <= t_min {
                return false;
            }
        }

        true
    }
}
