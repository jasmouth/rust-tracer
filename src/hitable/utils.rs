use rand::distributions::{Distribution, Uniform};
use vec3::{dot, Vec3};

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

/// Calculates the direction of a ray after reflecting off of a mirrored surface.
/// #### Arguments:
/// - `dir`: The initial direction of the ray
/// - `norm`: The surface normal of the mirror
pub fn reflect(dir: &Vec3, norm: &Vec3) -> Vec3 {
    *dir - (2.0 * dot(dir, norm) * (*norm))
}
