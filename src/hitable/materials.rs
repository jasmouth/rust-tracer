use hitable::hit_record::HitRecord;
use hitable::utils;
use ray::Ray;
use vec3::{dot, unit_vector, Vec3};

pub trait Material {
    /// Scatters a given ray; that is, a new ray is created that represents how the input ray
    /// would be scattered upon impact with the material.
    /// #### Returns
    /// - Tuple (Ray, Vec3, bool):
    ///   - Ray: The scattered ray,
    ///   - Vec3: The attenuation of the scattered ray,
    ///   - bool: Whether or not the input ray was successfully scattered
    fn scatter(&self, input_ray: &Ray, hit_record: &HitRecord) -> (Ray, Vec3, bool);
    fn box_clone(&self) -> Box<Material>;
}

impl Clone for Box<Material> {
    fn clone(&self) -> Box<Material> {
        self.box_clone()
    }
}

/// A Lambertian material is a "matte", or diffusely reflecting, surface.
#[derive(Copy, Clone, Debug)]
pub struct Lambertian {
    pub albedo: Vec3,
}

impl Lambertian {
    pub fn new() -> Self {
        Lambertian {
            albedo: Vec3::new(0.0, 0.0, 0.0),
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _input_ray: &Ray, hit_record: &HitRecord) -> (Ray, Vec3, bool) {
        let target =
            hit_record.hit_point + hit_record.normal + utils::random_point_in_unit_sphere();
        let scatteredRay = Ray {
            origin: hit_record.hit_point,
            direction: target - hit_record.hit_point,
        };
        let attenuation = Vec3 { e: self.albedo.e };
        (scatteredRay, attenuation, true)
    }

    fn box_clone(&self) -> Box<Material> {
        Box::new((*self).clone())
    }
}

/// A metallic surface. The fuzziness field dictates how glossy (or polished) the surface appears.
#[derive(Copy, Clone, Debug)]
pub struct Metal {
    pub albedo: Vec3,
    pub fuzziness: f64,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f64) -> Self {
        Metal {
            albedo,
            fuzziness: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, input_ray: &Ray, hit_record: &HitRecord) -> (Ray, Vec3, bool) {
        let reflected_ray = utils::reflect(&unit_vector(input_ray.direction), &hit_record.normal);
        let scatteredRay = Ray {
            origin: hit_record.hit_point,
            direction: reflected_ray + self.fuzziness * utils::random_point_in_unit_sphere(),
        };
        let attenuation = Vec3 { e: self.albedo.e };
        // If the length of the scattered ray in relation to the surface normal is <= 0,
        // the ray has been scattered under the object's surface.
        let didScatter = dot(&scatteredRay.direction, &hit_record.normal) > 0.0;
        (scatteredRay, attenuation, didScatter)
    }

    fn box_clone(&self) -> Box<Material> {
        Box::new((*self).clone())
    }
}
