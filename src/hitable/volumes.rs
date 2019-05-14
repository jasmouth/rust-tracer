use bounding_boxes::axis_aligned::AxisAlignedBoundingBox;
use hitable::hit_record::HitRecord;
use hitable::hitable::Hitable;
use material::material::Material;
use material::materials::Isotropic;
use rand::Rng;
use ray::Ray;
use std::f64::MAX as FLOAT_MAX;
use std::f64::MIN as FLOAT_MIN;
use texture::perlin::Perlin;
use texture::texture::Texture;
use vec3::Vec3;

/// Represents a participating medium with a constant density
#[derive(Clone)]
pub struct ConstantMedium {
    /// The boundary within which the medium is contained
    boundary: Box<Hitable>,
    /// The density of the medium
    density: f64,
    /// Describes the way light is scattered at any given point
    /// within the medium. For our purposes, this description is
    /// given as a Material (which inherently scatters light)
    phase_func: Box<Material>,
}

impl ConstantMedium {
    pub fn new(boundary: Box<Hitable>, density: f64, texture: Box<Texture>) -> Self {
        let phase_func = Box::new(Isotropic { albedo: texture });
        ConstantMedium {
            boundary,
            density,
            phase_func,
        }
    }
}

impl Hitable for ConstantMedium {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let ref mut rec_1 = HitRecord::new();
        let ref mut rec_2 = HitRecord::new();

        if self.boundary.hit(ray, FLOAT_MIN, FLOAT_MAX, rec_1) {
            if self.boundary.hit(ray, rec_1.t + 0.00001, FLOAT_MAX, rec_2) {
                if rec_1.t < t_min {
                    rec_1.t = t_min;
                }
                if rec_2.t > t_max {
                    rec_2.t = t_max;
                }
                if rec_1.t >= rec_2.t {
                    return false;
                }
                if rec_1.t < 0f64 {
                    rec_1.t = 0f64;
                }

                // The probabilistic distance that a Ray would need to travel through the
                // medium before it would be reflected.
                let hit_distance = -(1.0 - rand::thread_rng().gen::<f64>()).ln() / self.density;
                let ray_length = ray.direction.length();
                // The actual distance that the ray travels through the medium.
                let dist_inside_boundary = (rec_2.t - rec_1.t) * ray_length;

                if hit_distance < dist_inside_boundary {
                    rec.t = rec_1.t + (hit_distance / ray_length);
                    rec.hit_point = ray.point_at_param(rec.t);
                    rec.normal = Vec3::new(1.0, 0.0, 0.0); // An arbitrary direction
                    rec.material = Some(self.phase_func.clone());
                    return true;
                }
            }
        }

        false
    }

    fn bounding_box(&self, start_time: f64, end_time: f64) -> Option<AxisAlignedBoundingBox> {
        self.boundary.bounding_box(start_time, end_time)
    }

    fn box_clone(&self) -> Box<Hitable> {
        Box::new((*self).clone())
    }
}

/// Represents a participating medium with a variable density
#[derive(Clone)]
pub struct VariableMedium {
    /// The boundary within which the medium is contained
    boundary: Box<Hitable>,
    /// The maximum density of the medium
    max_density: f64,
    /// A noise function (which maps a Vec3 to a real number)
    /// which is used to vary the density of the medium
    noise_func: Perlin,
    /// Describes the way light is scattered at any given point
    /// within the medium. For our purposes, this description is
    /// given as a Material (which inherently scatters light)
    phase_func: Box<Material>,
}

impl VariableMedium {
    pub fn new(boundary: Box<Hitable>, max_density: f64, texture: Box<Texture>) -> Self {
        let phase_func = Box::new(Isotropic { albedo: texture });
        VariableMedium {
            boundary,
            max_density,
            noise_func: Perlin::new(),
            phase_func,
        }
    }
}

impl Hitable for VariableMedium {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let ref mut rec_1 = HitRecord::new();
        let ref mut rec_2 = HitRecord::new();

        if self.boundary.hit(ray, FLOAT_MIN, FLOAT_MAX, rec_1) {
            if self.boundary.hit(ray, rec_1.t + 0.00001, FLOAT_MAX, rec_2) {
                if rec_1.t < t_min {
                    rec_1.t = t_min;
                }
                if rec_2.t > t_max {
                    rec_2.t = t_max;
                }
                if rec_1.t >= rec_2.t {
                    return false;
                }
                if rec_1.t < 0f64 {
                    rec_1.t = 0f64;
                }

                let ray_length = ray.direction.length();
                let unit_length_direction = ray.direction / ray_length;
                // The actual distance that the ray travels through the medium.
                let dist_inside_boundary = (rec_2.t - rec_1.t) * ray_length;
                let mut hit_distance = 0.0;
                let mut rng = rand::thread_rng();

                loop {
                    let x = rng.gen::<f64>();
                    let y = rng.gen::<f64>();
                    hit_distance += -(1.0 - x).ln() / self.max_density;
                    let temp_hit_point = ray.origin + hit_distance * unit_length_direction;
                    if y < self.noise_func.turbulance(temp_hit_point, 8, 256.0) / self.max_density {
                        break;
                    }
                }

                if hit_distance < dist_inside_boundary {
                    rec.t = rec_1.t + (hit_distance / ray_length);
                    rec.hit_point = ray.point_at_param(rec.t);
                    rec.normal = Vec3::new(1.0, 0.0, 0.0); // An arbitrary direction
                    rec.material = Some(self.phase_func.clone());
                    return true;
                }
            }
        }

        false
    }

    fn bounding_box(&self, start_time: f64, end_time: f64) -> Option<AxisAlignedBoundingBox> {
        self.boundary.bounding_box(start_time, end_time)
    }

    fn box_clone(&self) -> Box<Hitable> {
        Box::new((*self).clone())
    }
}
