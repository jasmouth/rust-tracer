use ray::Ray;
use vec3::{cross, unit_vector, Vec3};

pub struct Camera {
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub origin: Vec3,
}

impl Camera {
    /// Creates a new camera.
    /// #### Arguments:
    /// - `look_from`: The point from which to center the camera
    /// - `look_at`: The point at which the camera is aimed
    /// - `view_up`: The "up" direction of the camera
    /// - `vert_fov`: The vertical field of view in degrees
    /// - `aspect_ratio`: Aspect ratio to use for the camera
    pub fn new(
        look_from: Vec3,
        look_at: Vec3,
        view_up: Vec3,
        vert_fov: f64,
        aspect_ratio: f64,
    ) -> Self {
        let theta = vert_fov * std::f64::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect_ratio * half_height;
        let w = unit_vector(look_from - look_at);
        let u = unit_vector(cross(&view_up, &w));
        let v = cross(&w, &u);
        Camera {
            lower_left_corner: look_from - (half_width * u) - (half_height * v) - w,
            horizontal: 2.0 * half_width * u,
            vertical: 2.0 * half_height * v,
            origin: look_from,
        }
    }

    pub fn create_ray(&self, u: f64, v: f64) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.lower_left_corner + (u * self.horizontal) + (v * self.vertical)
                - self.origin,
        }
    }
}
