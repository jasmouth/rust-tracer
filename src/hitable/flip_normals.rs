use bounding_boxes::axis_aligned::AxisAlignedBoundingBox;
use hitable::hit_record::HitRecord;
use hitable::hitable::Hitable;
use ray::Ray;
use std::sync::Arc;

/// The only purpose this struct serves is to wrap
/// a Hitable, and reverse its surface normals.
#[derive(Clone)]
pub struct FlipNormals {
    hitable: Arc<Hitable>,
}

impl FlipNormals {
    pub fn new(hitable: Arc<Hitable>) -> Self {
        FlipNormals { hitable }
    }
}

impl Hitable for FlipNormals {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        if self.hitable.hit(ray, t_min, t_max, rec) {
            rec.normal = -rec.normal;
            return true;
        }
        false
    }

    fn bounding_box(&self, start_time: f64, end_time: f64) -> Option<AxisAlignedBoundingBox> {
        self.hitable.bounding_box(start_time, end_time)
    }
}
