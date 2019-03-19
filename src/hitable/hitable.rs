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
}
