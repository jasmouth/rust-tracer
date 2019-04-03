use ray::Ray;
use vec3::Vec3;

/// Represents an Axis-aligned Bounding Box.
#[derive(Copy, Clone, Debug)]
pub struct AxisAlignedBoundingBox {
    pub min_bound: Vec3,
    pub max_bound: Vec3,
    pub bounds: [Vec3; 2],
}

impl AxisAlignedBoundingBox {
    /// Creates a new AxisAlignedBoundingBox with the given min_bound and max_bound
    pub fn new(min_bound: Vec3, max_bound: Vec3) -> Self {
        AxisAlignedBoundingBox {
            min_bound,
            max_bound,
            bounds: [min_bound, max_bound],
        }
    }

    /// Determines whether the given ray intersects this bounding box
    ///
    /// The method used is taken from Amy Williams et al. `An Efficient and Robust
    /// Ray-Box Intersection Algorithm`
    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> bool {
        let mut _t_min =
            (self.bounds[ray.sign[0] as usize].x() - ray.origin.x()) * ray.invert_direction.x();
        let mut _t_max =
            (self.bounds[1 - ray.sign[0] as usize].x() - ray.origin.x()) * ray.invert_direction.x();
        let t_y_min =
            (self.bounds[ray.sign[1] as usize].y() - ray.origin.y()) * ray.invert_direction.y();
        let t_y_max =
            (self.bounds[1 - ray.sign[1] as usize].y() - ray.origin.y()) * ray.invert_direction.y();
        if (_t_min > t_y_max) || (t_y_min > _t_max) {
            return false;
        }
        if t_y_min > _t_min {
            _t_min = t_y_min;
        }
        if t_y_max < _t_max {
            _t_max = t_y_max;
        }
        let t_z_min =
            (self.bounds[ray.sign[2] as usize].z() - ray.origin.z()) * ray.invert_direction.z();
        let t_z_max =
            (self.bounds[1 - ray.sign[2] as usize].z() - ray.origin.z()) * ray.invert_direction.z();
        if (_t_min > t_z_max) || (t_z_min > _t_max) {
            return false;
        }
        if t_z_min > _t_min {
            _t_min = t_z_min;
        }
        if t_z_max < _t_max {
            _t_max = t_z_max;
        }

        (_t_min < t_max) && (_t_max > t_min)
    }
}
