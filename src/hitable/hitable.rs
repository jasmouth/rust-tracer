use hitable::hit_record::HitRecord;
use ray::Ray;

pub trait Hitable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}
