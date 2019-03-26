use std::marker::{Send, Sync};
use vec3::Vec3;

pub trait Texture: Send + Sync {
    /// Calculates the value for the texture given the parameters
    /// `u` and `v` at the hit-point of a Ray
    fn value(&self, u: f64, v: f64, hit_point: &Vec3) -> Vec3;
    fn box_clone(&self) -> Box<Texture>;
}

impl Clone for Box<Texture> {
    fn clone(&self) -> Box<Texture> {
        self.box_clone()
    }
}
