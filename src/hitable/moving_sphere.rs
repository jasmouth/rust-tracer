use bounding_boxes::axis_aligned::AxisAlignedBoundingBox;
use bounding_boxes::utils;
use hitable::hit_record::HitRecord;
use hitable::hitable::Hitable;
use material::material::Material;
use ray::Ray;
use vec3::{dot, Vec3};

/// Represents a sphere that can move from one point to another
/// over a specific timeframe
#[derive(Clone)]
pub struct MovingSphere {
    pub radius: f64,
    pub start_center: Vec3,
    pub end_center: Vec3,
    pub start_time: f64,
    pub end_time: f64,
    pub material: Box<Material>,
}

impl MovingSphere {
    /// Calculates the sphere's center at the given time
    pub fn get_center(&self, time: f64) -> Vec3 {
        self.start_center
            + ((time - self.start_time) / (self.end_time - self.start_time))
                * (self.end_center - self.start_center)
    }
}

impl Hitable for MovingSphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = ray.origin - self.get_center(ray.time);
        let a = dot(&ray.direction, &ray.direction);
        let b = dot(&oc, &ray.direction);
        let c = dot(&oc, &oc) - (self.radius * self.radius);
        let discriminant = (b * b) - (a * c);
        if discriminant > 0.0 {
            let mut temp = (-b - discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.hit_point = ray.point_at_param(rec.t);
                rec.normal = (rec.hit_point - self.get_center(ray.time)) / self.radius;
                rec.material = Some(self.material.clone());
                return true;
            }
            temp = (-b + discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.hit_point = ray.point_at_param(rec.t);
                rec.normal = (rec.hit_point - self.get_center(ray.time)) / self.radius;
                rec.material = Some(self.material.clone());
                return true;
            }
            return false;
        } else {
            return false;
        }
    }

    fn bounding_box(&self, start_time: f64, end_time: f64) -> Option<AxisAlignedBoundingBox> {
        let start_box = AxisAlignedBoundingBox::new(
            self.get_center(start_time) - self.radius,
            self.get_center(start_time) + self.radius,
        );
        let end_box = AxisAlignedBoundingBox::new(
            self.get_center(end_time) - self.radius,
            self.get_center(end_time) + self.radius,
        );
        Some(utils::calc_surrounding_box(&start_box, &end_box))
    }

    fn box_clone(&self) -> Box<Hitable> {
        Box::new((*self).clone())
    }
}
