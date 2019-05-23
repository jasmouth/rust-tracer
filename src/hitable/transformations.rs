use bounding_boxes::axis_aligned::AxisAlignedBoundingBox;
use hitable::hit_record::HitRecord;
use hitable::hitable::Hitable;
use ray::Ray;
use std::f64::consts::PI;
use std::f64::MAX as FLOAT_MAX;
use std::f64::MIN as FLOAT_MIN;
use std::sync::Arc;
use vec3::Vec3;

/// Wrapper struct that wraps a Hitable and shifts it by some given offset
#[derive(Clone)]
pub struct Translate {
    pub hitable: Arc<Hitable>,
    pub offset: Vec3,
}

impl Translate {
    /// Arguments:
    /// - `hitable`: The Hitable to be shifted
    /// - `offset`: The offset amount used to shift the Hitable
    pub fn new(hitable: Arc<Hitable>, offset: Vec3) -> Self {
        Translate { hitable, offset }
    }
}

impl Hitable for Translate {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let translated_ray = Ray::new(ray.origin - self.offset, ray.direction, ray.time);
        if self.hitable.hit(&translated_ray, t_min, t_max, rec) {
            rec.hit_point += self.offset;
            return true;
        }
        false
    }

    fn bounding_box(&self, start_time: f64, end_time: f64) -> Option<AxisAlignedBoundingBox> {
        match self.hitable.bounding_box(start_time, end_time) {
            Some(bound) => Some(AxisAlignedBoundingBox::new(
                bound.min_bound + self.offset,
                bound.max_bound + self.offset,
            )),
            None => None,
        }
    }
}

/// Wrapper struct that wraps a Hitable and rotates it about the Y axis
#[derive(Clone)]
pub struct RotateY {
    hitable: Arc<Hitable>,
    sin_theta: f64,
    cos_theta: f64,
    bounding_box: Option<AxisAlignedBoundingBox>,
}

impl RotateY {
    /// Arguments:
    /// - `hitable`: The Hitable to be rotated
    /// - `theta`: The angle (in degrees) to rotate the Hitable
    pub fn new(hitable: Arc<Hitable>, theta: f64) -> Self {
        let rads = (PI / 180.0) * theta;
        let sin_theta = rads.sin();
        let cos_theta = rads.cos();
        let bounding_box: AxisAlignedBoundingBox = hitable.bounding_box(0.0, 1.0).unwrap();
        let mut min = Vec3::new(FLOAT_MAX, FLOAT_MAX, FLOAT_MAX);
        let mut max = Vec3::new(FLOAT_MIN, FLOAT_MIN, FLOAT_MIN);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * bounding_box.max_bound.x()
                        + (1 - i) as f64 * bounding_box.min_bound.x();
                    let y = j as f64 * bounding_box.max_bound.y()
                        + (1 - j) as f64 * bounding_box.min_bound.y();
                    let z = k as f64 * bounding_box.max_bound.z()
                        + (1 - k) as f64 * bounding_box.min_bound.z();
                    // Rotated clockwise about the Y axis
                    let rotated_x = cos_theta * x + sin_theta * z;
                    let rotated_z = -sin_theta * x + cos_theta * z;
                    let temp_rotation = Vec3::new(rotated_x, y, rotated_z);

                    if temp_rotation.x() > max.x() {
                        max[0] = temp_rotation.x();
                    }
                    if temp_rotation.x() < min.x() {
                        min[0] = temp_rotation.x();
                    }
                    if temp_rotation.y() > max.y() {
                        max[1] = temp_rotation.y();
                    }
                    if temp_rotation.y() < min.y() {
                        min[1] = temp_rotation.y();
                    }
                    if temp_rotation.z() > max.z() {
                        max[2] = temp_rotation.z();
                    }
                    if temp_rotation.z() < min.z() {
                        min[2] = temp_rotation.z();
                    }
                }
            }
        }

        RotateY {
            hitable,
            sin_theta,
            cos_theta,
            bounding_box: Some(AxisAlignedBoundingBox::new(min, max)),
        }
    }
}

impl Hitable for RotateY {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut origin = ray.origin;
        let mut direction = ray.direction;
        // Rotated counter-clockwise about the Y axis
        origin[0] = self.cos_theta * ray.origin.x() - self.sin_theta * ray.origin.z();
        origin[2] = self.sin_theta * ray.origin.x() + self.cos_theta * ray.origin.z();
        direction[0] = self.cos_theta * ray.direction.x() - self.sin_theta * ray.direction.z();
        direction[2] = self.sin_theta * ray.direction.x() + self.cos_theta * ray.direction.z();
        let rotated_ray = Ray::new(origin, direction, ray.time);
        if self.hitable.hit(&rotated_ray, t_min, t_max, rec) {
            let mut hit_point = rec.hit_point;
            let mut normal = rec.normal;
            hit_point[0] = self.cos_theta * rec.hit_point.x() + self.sin_theta * rec.hit_point.z();
            hit_point[2] = -self.sin_theta * rec.hit_point.x() + self.cos_theta * rec.hit_point.z();
            normal[0] = self.cos_theta * rec.normal.x() + self.sin_theta * rec.normal.z();
            normal[2] = -self.sin_theta * rec.normal.x() + self.cos_theta * rec.normal.z();
            rec.hit_point = hit_point;
            rec.normal = normal;
            return true;
        } else {
            return false;
        }
    }

    fn bounding_box(&self, _start_time: f64, _end_time: f64) -> Option<AxisAlignedBoundingBox> {
        self.bounding_box
    }
}
