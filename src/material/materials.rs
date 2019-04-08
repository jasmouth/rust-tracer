use hitable::hit_record::HitRecord;
use hitable::utils;
use material::material::Material;
use rand::distributions::{Distribution, Uniform};
use ray::Ray;
use texture::texture::Texture;
use texture::textures::ConstantTexture;
use vec3::{dot, unit_vector, Vec3};

/// A Lambertian material is a "matte", or diffusely reflecting, surface.
#[derive(Clone)]
pub struct Lambertian {
    pub albedo: Box<Texture>,
}

impl Lambertian {
    pub fn new() -> Self {
        Lambertian {
            albedo: Box::new(ConstantTexture::new(Vec3::new(0.0, 0.0, 0.0))),
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, input_ray: &Ray, hit_record: &HitRecord) -> (Ray, Vec3, bool) {
        let target =
            hit_record.hit_point + hit_record.normal + utils::random_point_in_unit_sphere();
        let scattered_ray = Ray::new(
            hit_record.hit_point,
            target - hit_record.hit_point,
            input_ray.time,
        );
        let attenuation = self
            .albedo
            .value(hit_record.u, hit_record.v, &hit_record.hit_point);
        (scattered_ray, attenuation, true)
    }

    fn box_clone(&self) -> Box<Material> {
        Box::new((*self).clone())
    }
}

/// A metallic surface. The fuzziness field dictates how glossy (or polished) the surface appears.
#[derive(Clone)]
pub struct Metal {
    pub albedo: Box<Texture>,
    pub fuzziness: f64,
}

impl Metal {
    pub fn new(albedo: Box<Texture>, fuzz: f64) -> Self {
        Metal {
            albedo,
            fuzziness: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, input_ray: &Ray, hit_record: &HitRecord) -> (Ray, Vec3, bool) {
        let reflected_ray = utils::reflect(&unit_vector(input_ray.direction), &hit_record.normal);
        let scattered_ray = Ray::new(
            hit_record.hit_point,
            reflected_ray + self.fuzziness * utils::random_point_in_unit_sphere(),
            input_ray.time,
        );
        let attenuation = self
            .albedo
            .value(hit_record.u, hit_record.v, &hit_record.hit_point);
        // If the length of the scattered ray in relation to the surface normal is <= 0,
        // the ray has been scattered under the object's surface.
        let did_scatter = dot(&scattered_ray.direction, &hit_record.normal) > 0.0;
        (scattered_ray, attenuation, did_scatter)
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
            scattered_ray = Ray::new(
                hit_record.hit_point,
                utils::reflect(&input_ray.direction, &hit_record.normal),
                input_ray.time,
            );
        } else {
            scattered_ray = Ray::new(hit_record.hit_point, refracted_ray, input_ray.time);
        };

        (scattered_ray, attenuation, true)
    }

    fn box_clone(&self) -> Box<Material> {
        Box::new((*self).clone())
    }
}

/// A material that emits diffused (i.e. non-concentrated) light
#[derive(Clone)]
pub struct DiffuseLight {
    texture: Box<Texture>,
}

impl DiffuseLight {
    pub fn new(texture: Box<Texture>) -> Self {
        DiffuseLight { texture }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, input_ray: &Ray, _hit_record: &HitRecord) -> (Ray, Vec3, bool) {
        let blank_ray = Ray::new(input_ray.direction, input_ray.origin, 0.0);
        (blank_ray, Vec3::new(0.0, 0.0, 0.0), false)
    }

    fn emit(&self, u: f64, v: f64, hit_point: &Vec3) -> Vec3 {
        self.texture.value(u, v, hit_point)
    }

    fn box_clone(&self) -> Box<Material> {
        Box::new((*self).clone())
    }
}

/// A material that uniformly scatters light in all directions
#[derive(Clone)]
pub struct Isotropic {
    pub albedo: Box<Texture>,
}

impl Material for Isotropic {
    fn scatter(&self, input_ray: &Ray, hit_record: &HitRecord) -> (Ray, Vec3, bool) {
        let scattered_ray = Ray::new(
            hit_record.hit_point,
            utils::random_point_in_unit_sphere(),
            input_ray.time,
        );
        let attenuation = self
            .albedo
            .value(hit_record.u, hit_record.v, &hit_record.hit_point);
        (scattered_ray, attenuation, true)
    }

    fn box_clone(&self) -> Box<Material> {
        Box::new((*self).clone())
    }
}
