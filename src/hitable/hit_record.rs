use material::material::Material;
use vec3::Vec3;

pub struct HitRecord {
    pub t: f64,
    pub hit_point: Vec3,
    pub normal: Vec3,
    pub material: Option<Box<Material>>,
    pub u: f64,
    pub v: f64,
}

impl HitRecord {
    pub fn new() -> Self {
        HitRecord {
            t: -1.0,
            hit_point: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            material: None,
            u: 0.0,
            v: 0.0,
        }
    }

    /// Updates the record with the values existing in `other`
    pub fn from(&mut self, other: &mut HitRecord) {
        self.t = other.t;
        self.hit_point = other.hit_point;
        self.normal = other.normal;
        self.material = other.material.take();
        self.u = other.u;
        self.v = other.v;
    }
}
