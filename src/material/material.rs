use hitable::hit_record::HitRecord;
use ray::Ray;
use std::marker::{Send, Sync};
use vec3::Vec3;

pub trait Material: Send + Sync {
    /// Scatters a given ray; that is, a new ray is created that represents how the input ray
    /// would be scattered upon impact with the material.
    /// #### Returns
    /// - Tuple (Ray, Vec3, bool):
    ///   - Ray: The scattered ray,
    ///   - Vec3: The attenuation of the scattered ray,
    ///   - bool: Whether or not the input ray was successfully scattered
    fn scatter(&self, input_ray: &Ray, hit_record: &HitRecord) -> (Ray, Vec3, bool);
    fn box_clone(&self) -> Box<Material>;
}

impl Clone for Box<Material> {
    fn clone(&self) -> Box<Material> {
        self.box_clone()
    }
}