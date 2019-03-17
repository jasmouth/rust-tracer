use hitable::hit_record::HitRecord;
use hitable::hitable::Hitable;
use hitable::materials::Material;
use ray::Ray;
use std::clone::Clone;
use vec3::{dot, Vec3};

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Box<Material>,
}

impl Hitable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = ray.origin - self.center;
        let a = dot(&ray.direction, &ray.direction);
        let b = dot(&oc, &ray.direction);
        let c = dot(&oc, &oc) - (self.radius * self.radius);
        let discriminant = (b * b) - (a * c);
        if discriminant > 0.0 {
            let mut temp = (-b - discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.hit_point = ray.point_at_param(rec.t);
                rec.normal = (rec.hit_point - self.center) / self.radius;
                rec.material = Some(self.material.clone());
                return true;
            }
            temp = (-b + discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.hit_point = ray.point_at_param(rec.t);
                rec.normal = (rec.hit_point - self.center) / self.radius;
                rec.material = Some(self.material.clone());
                return true;
            }
            return false;
        } else {
            return false;
        }
    }
}
