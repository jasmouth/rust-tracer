use hitable::hit_record::HitRecord;
use hitable::utils;
use rand::distributions::{Distribution, Uniform};
use ray::Ray;
use std::marker::{Send, Sync};
use vec3::{dot, unit_vector, Vec3};

pub trait Material: Send + Sync {
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

/// A surface that splits a light ray into both a refracted and reflected ray (e.g. glass, water, etc.).
///
/// Note that only one ray is generated per interaction; the choice between reflected and refracted
/// is chosen randomly.
#[derive(Copy, Clone, Debug)]
pub struct Dielectric {
    pub refractive_index: f64,
}

impl Dielectric {
    pub fn new(refractive_index: f64) -> Self {
        Dielectric { refractive_index }
    }
}

impl Material for Dielectric {
    fn scatter(&self, input_ray: &Ray, hit_record: &HitRecord) -> (Ray, Vec3, bool) {
        let range = Uniform::new_inclusive(0.0, 1.0);
        let mut rng = rand::thread_rng();
        // The glass surface does not absorb anything, so attenuation is set to 1
        let attenuation = Vec3::new(1.0, 1.0, 1.0);
        let dot_prod = dot(&input_ray.direction, &hit_record.normal);
        // n1/n2 -> ray enters medium 2 from medium 1
        let ni_over_nt: f64;
        let cosine: f64;
        let outward_normal: Vec3;

        // If dot_prod is > 0, this means that the ray is coming from inside the object
        if dot_prod > 0.0 {
            outward_normal = -hit_record.normal;
            ni_over_nt = self.refractive_index;
            cosine = self.refractive_index * dot_prod / input_ray.direction.length();
        } else {
            outward_normal = hit_record.normal;
            ni_over_nt = 1.0 / self.refractive_index;
            cosine = -dot_prod / input_ray.direction.length();
        }

        let refracted_ray: Vec3;
        let reflect_probability =
            match utils::refract(&input_ray.direction, &outward_normal, ni_over_nt) {
                Some(refracted) => {
                    refracted_ray = refracted;
                    utils::schlick_approx(cosine, self.refractive_index)
                }
                None => {
                    refracted_ray = Vec3::new(0.0, 0.0, 0.0);
                    1.0
                }
            };

        let scattered_ray: Ray;
        if range.sample(&mut rng) <= reflect_probability {
            scattered_ray = Ray {
                origin: hit_record.hit_point,
                direction: utils::reflect(&input_ray.direction, &hit_record.normal),
            }
        } else {
            scattered_ray = Ray {
                origin: hit_record.hit_point,
                direction: refracted_ray,
            }
        };

        (scattered_ray, attenuation, true)
    }

    fn box_clone(&self) -> Box<Material> {
        Box::new((*self).clone())
    }
}
