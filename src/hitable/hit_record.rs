use hitable::materials::Material;
use vec3::Vec3;

pub struct HitRecord {
    pub t: f64,
    pub hit_point: Vec3,
    pub normal: Vec3,
    pub material: Option<Box<Material>>,
}

impl HitRecord {
    pub fn new() -> Self {
        HitRecord {
            t: -1.0,
            hit_point: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            material: None,
        }
    }
}
