use bounding_boxes::axis_aligned::AxisAlignedBoundingBox;
use hitable::hit_record::HitRecord;
use hitable::hitable::Hitable;
use hitable::utils;
use material::material::Material;
use ray::Ray;
use std::sync::Arc;
use vec3::{dot, Vec3};

/// Represents a stationary sphere
#[derive(Clone)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Arc<Material>,
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
                rec.material = Some(Arc::clone(&self.material));
                let (u, v) = utils::get_sphere_uv(&rec.normal);
                rec.u = u;
                rec.v = v;
                return true;
            }
            temp = (-b + discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.hit_point = ray.point_at_param(rec.t);
                rec.normal = (rec.hit_point - self.center) / self.radius;
                rec.material = Some(Arc::clone(&self.material));
                let (u, v) = utils::get_sphere_uv(&rec.normal);
                rec.u = u;
                rec.v = v;
                return true;
            }
            return false;
        } else {
            return false;
        }
    }

    fn bounding_box(&self, _start_time: f64, _end_time: f64) -> Option<AxisAlignedBoundingBox> {
        Some(AxisAlignedBoundingBox::new(
            self.center - self.radius,
            self.center + self.radius,
        ))
    }
}
