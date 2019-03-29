use bounding_boxes::axis_aligned::AxisAlignedBoundingBox;
use hitable::hit_record::HitRecord;
use hitable::hitable::Hitable;
use material::material::Material;
use ray::Ray;
use vec3::Vec3;

/// Represents a rectangle aligned along the X-Y axis
#[derive(Clone)]
pub struct XYRect {
    pub material: Box<Material>,
    pub x_0: f64,
    pub x_1: f64,
    pub y_0: f64,
    pub y_1: f64,
    pub k: f64,
}

impl Hitable for XYRect {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let t = (self.k - ray.origin.z()) / ray.direction.z();
        if t < t_min || t > t_max {
            return false;
        }
        let x = ray.origin.x() + t * ray.direction.x();
        let y = ray.origin.y() + t * ray.direction.y();
        if x < self.x_0 || x > self.x_1 || y < self.y_0 || y > self.y_1 {
            return false;
        }
        rec.t = t;
        rec.hit_point = ray.point_at_param(t);
        rec.normal = Vec3::new(0.0, 0.0, 1.0);
        rec.material = Some(self.material.clone());
        let (u, v) = (
            (x - self.x_0) / (self.x_1 - self.x_0),
            (y - self.y_0) / (self.y_1 - self.y_0),
        );
        rec.u = u;
        rec.v = v;
        true
    }

    fn bounding_box(&self, _start_time: f64, _end_time: f64) -> Option<AxisAlignedBoundingBox> {
        Some(AxisAlignedBoundingBox::new(
            Vec3::new(self.x_0, self.y_0, self.k - 0.0001),
            Vec3::new(self.x_1, self.y_1, self.k + 0.0001),
        ))
    }

    fn box_clone(&self) -> Box<Hitable> {
        Box::new((*self).clone())
    }
}

/// Represents a rectangle aligned along the X-Z axis
#[derive(Clone)]
pub struct XZRect {
    pub material: Box<Material>,
    pub x_0: f64,
    pub x_1: f64,
    pub z_0: f64,
    pub z_1: f64,
    pub k: f64,
}

impl Hitable for XZRect {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let t = (self.k - ray.origin.y()) / ray.direction.y();
        if t < t_min || t > t_max {
            return false;
        }
        let x = ray.origin.x() + t * ray.direction.x();
        let z = ray.origin.z() + t * ray.direction.z();
        if x < self.x_0 || x > self.x_1 || z < self.z_0 || z > self.z_1 {
            return false;
        }
        rec.t = t;
        rec.hit_point = ray.point_at_param(t);
        rec.normal = Vec3::new(0.0, 1.0, 0.0);
        rec.material = Some(self.material.clone());
        let (u, v) = (
            (x - self.x_0) / (self.x_1 - self.x_0),
            (z - self.z_0) / (self.z_1 - self.z_0),
        );
        rec.u = u;
        rec.v = v;
        true
    }

    fn bounding_box(&self, _start_time: f64, _end_time: f64) -> Option<AxisAlignedBoundingBox> {
        Some(AxisAlignedBoundingBox::new(
            Vec3::new(self.x_0, self.k - 0.0001, self.z_0),
            Vec3::new(self.x_1, self.k + 0.0001, self.z_1),
        ))
    }

    fn box_clone(&self) -> Box<Hitable> {
        Box::new((*self).clone())
    }
}

/// Represents a rectangle aligned along the Y-Z axis
#[derive(Clone)]
pub struct YZRect {
    pub material: Box<Material>,
    pub y_0: f64,
    pub y_1: f64,
    pub z_0: f64,
    pub z_1: f64,
    pub k: f64,
}

impl Hitable for YZRect {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let t = (self.k - ray.origin.x()) / ray.direction.x();
        if t < t_min || t > t_max {
            return false;
        }
        let y = ray.origin.y() + t * ray.direction.y();
        let z = ray.origin.z() + t * ray.direction.z();
        if z < self.z_0 || z > self.z_1 || y < self.y_0 || y > self.y_1 {
            return false;
        }
        rec.t = t;
        rec.hit_point = ray.point_at_param(t);
        rec.normal = Vec3::new(1.0, 0.0, 0.0);
        rec.material = Some(self.material.clone());
        let (u, v) = (
            (y - self.y_0) / (self.y_1 - self.y_0),
            (z - self.z_0) / (self.z_1 - self.z_0),
        );
        rec.u = u;
        rec.v = v;
        true
    }

    fn bounding_box(&self, _start_time: f64, _end_time: f64) -> Option<AxisAlignedBoundingBox> {
        Some(AxisAlignedBoundingBox::new(
            Vec3::new(self.k - 0.0001, self.y_0, self.z_0),
            Vec3::new(self.k + 0.0001, self.y_1, self.z_1),
        ))
    }

    fn box_clone(&self) -> Box<Hitable> {
        Box::new((*self).clone())
    }
}
