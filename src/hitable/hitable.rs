use bounding_boxes::axis_aligned::AxisAlignedBoundingBox;
use hitable::hit_record::HitRecord;
use ray::Ray;
use std::marker::{Send, Sync};

/// A trait declaring that an object can be hit by a ray.
pub trait Hitable: Send + Sync {
    /// Determines whether the given ray will hit the Hitable object.
    /// #### Arguments:
    /// - `ray`: The ray to check for collision
    /// - `t_min`: A lower bound on the Ray parameter `t`
    /// - `t_max`: An upper bound on the Ray parameter `t`
    /// - `rec`: A record of the ray's collision is written to this parameter
    ///
    /// #### Returns:
    /// - bool: Whether the given ray successfully hit the object.
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;

    /// Computes a bounding box for the object.
    /// #### Arguments:
    /// - `start_time`: The beginning of a moving object's movement timeframe
    /// - `end_time`: The end of a moving object's movement timeframe
    ///
    /// #### Returns:
    /// - An Optional containing the computed bounding box, or None if no box
    /// can be computed.
    fn bounding_box(&self, start_time: f64, end_time: f64) -> Option<AxisAlignedBoundingBox>;
    fn box_clone(&self) -> Box<Hitable>;
}

impl Clone for Box<Hitable> {
    fn clone(&self) -> Box<Hitable> {
        self.box_clone()
    }
}
