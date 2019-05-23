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
    /// Calculates a light's emitted color value.
    /// #### Arguments
    /// - `u`: Texture coordinate (u,_)
    /// - `v`: Texture coordinate (_,v)
    /// - `hit_point`: The point at which a Ray hits the Material
    fn emit(&self, u: f64, v: f64, hit_point: &Vec3) -> Vec3 {
        #![allow(unused_variables)]
        Vec3::new(0.0, 0.0, 0.0)
    }
}
