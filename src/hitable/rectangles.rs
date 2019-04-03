use bounding_boxes::axis_aligned::AxisAlignedBoundingBox;
use hitable::flip_normals::FlipNormals;
use hitable::hit_record::HitRecord;
use hitable::hitable::Hitable;
use hitable::hitable_list::HitableList;
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

/// Represents a block (i.e. a six-sided cuboid)
#[derive(Clone)]
pub struct AxisAlignedBlock {
    pub p_min: Vec3,
    pub p_max: Vec3,
    pub sides: HitableList,
}

impl AxisAlignedBlock {
    /// Constructs a new AxisAlignedBlock
    /// #### Arguments:
    /// - `p_min`: The plane to use for the lower bound of the box
    /// - `p_max`: The plane to use for the upper bound of the box
    /// - `material`: The material to use for the sides of the box
    pub fn new(p_min: Vec3, p_max: Vec3, material: Box<Material>) -> Self {
        let left_wall = Box::new(YZRect {
            material: material.clone(),
            y_0: p_min.y(),
            y_1: p_max.y(),
            z_0: p_min.z(),
            z_1: p_max.z(),
            k: p_max.x(),
        });
        let right_wall = Box::new(FlipNormals::new(Box::new(YZRect {
            material: material.clone(),
            y_0: p_min.y(),
            y_1: p_max.y(),
            z_0: p_min.z(),
            z_1: p_max.z(),
            k: p_min.x(),
        })));
        let back_wall = Box::new(XYRect {
            material: material.clone(),
            x_0: p_min.x(),
            x_1: p_max.x(),
            y_0: p_min.y(),
            y_1: p_max.y(),
            k: p_max.z(),
        });
        let front_wall = Box::new(FlipNormals::new(Box::new(XYRect {
            material: material.clone(),
            x_0: p_min.x(),
            x_1: p_max.x(),
            y_0: p_min.y(),
            y_1: p_max.y(),
            k: p_min.z(),
        })));
        let floor = Box::new(FlipNormals::new(Box::new(XZRect {
            material: material.clone(),
            x_0: p_min.x(),
            x_1: p_max.x(),
            z_0: p_min.z(),
            z_1: p_max.z(),
            k: p_min.y(),
        })));
        let ceiling = Box::new(XZRect {
            material: material.clone(),
            x_0: p_min.x(),
            x_1: p_max.x(),
            z_0: p_min.z(),
            z_1: p_max.z(),
            k: p_max.y(),
        });

        AxisAlignedBlock {
            p_min,
            p_max,
            sides: HitableList {
                list: vec![left_wall, right_wall, back_wall, front_wall, floor, ceiling],
            },
        }
    }
}

impl Hitable for AxisAlignedBlock {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        self.sides.hit(ray, t_min, t_max, rec)
    }

    fn bounding_box(&self, _start_time: f64, _end_time: f64) -> Option<AxisAlignedBoundingBox> {
        Some(AxisAlignedBoundingBox::new(self.p_min, self.p_max))
    }

    fn box_clone(&self) -> Box<Hitable> {
        Box::new((*self).clone())
    }
}
