use rand::distributions::{Distribution, Uniform};
use vec3::{dot, unit_vector, Vec3};

/// Generates a random point in a unit-radius sphere
pub fn random_point_in_unit_sphere() -> Vec3 {
    let range = Uniform::new_inclusive(0.0, 1.0);
    let mut rng = rand::thread_rng();
    let mut point;
    loop {
        point =
            2.0 * Vec3::new(
                range.sample(&mut rng),
                range.sample(&mut rng),
                range.sample(&mut rng),
            ) - Vec3::new(1.0, 1.0, 1.0);
        if dot(&point, &point) < 1.0 {
            break;
        }
    }

    point
}

/// Generates a random point in a unit-radius disk
pub fn random_point_in_unit_disk() -> Vec3 {
    let range = Uniform::new_inclusive(0.0, 1.0);
    let mut rng = rand::thread_rng();
    let mut point;
    loop {
        point = 2.0 * Vec3::new(range.sample(&mut rng), range.sample(&mut rng), 0.0)
            - Vec3::new(1.0, 1.0, 0.0);
        if dot(&point, &point) < 1.0 {
            break;
        }
    }

    point
}

/// Calculates the direction of a ray after reflecting off of a mirrored surface.
/// #### Arguments:
/// - `dir`: The initial direction of the ray
/// - `norm`: The surface normal of the mirror
pub fn reflect(dir: &Vec3, norm: &Vec3) -> Vec3 {
    *dir - (2.0 * dot(dir, norm) * (*norm))
}

/// Calculates the direction of a ray after being refracted through a dielectric material
/// #### Arguments:
/// - `dir`: The initial direction of the ray
/// - `norm`: The surface normal of the mirror
/// - `ni_over_nt`: The refractive index of medium 1 divided by the refractive index of medium 2 (i.e. n1/n2)
pub fn refract(dir: &Vec3, norm: &Vec3, ni_over_nt: f64) -> Option<Vec3> {
    let unit_vec = unit_vector(*dir);
    let dt = dot(&unit_vec, norm);
    let discriminant = 1.0 - (ni_over_nt * ni_over_nt * (1.0 - dt * dt));
    return if discriminant > 0.0 {
        Some(ni_over_nt * (unit_vec - (*norm) * dt) - ((*norm) * discriminant.sqrt()))
    } else {
        None
    };
}

/// Calculates Schlick's approximation for specular reflection
/// #### General Form:
/// - R(theta) = R_0 + (1 - R_0) * (1 - cos(theta))^5
///   - where R_0 = ((n1 - n2) / (n1 + n2))^2
/// - Note that in this case, the first medium is air which has a refractive index of 1 (i.e. n1 = 1)
pub fn schlick_approx(cosine: f64, refractive_index: f64) -> f64 {
    let mut r0 = (1.0 - refractive_index) / (1.0 + refractive_index);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
