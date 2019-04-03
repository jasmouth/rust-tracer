use hitable::utils;
use rand::Rng;
use ray::Ray;
use vec3::{cross, unit_vector, Vec3};

#[derive(Copy, Clone)]
pub struct Camera {
    pub lens_radius: f64,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub origin: Vec3,
    pub vertical: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub shutter_open_time: f64,
    pub shutter_close_time: f64,
}

impl Camera {
    /// Creates a new camera.
    /// #### Arguments:
    /// - `look_from`: The point from which to center the camera
    /// - `look_at`: The point at which the camera is aimed
    /// - `view_up`: The "up" direction of the camera
    /// - `vert_fov`: The vertical field of view in degrees
    /// - `aspect_ratio`: Aspect ratio to use for the camera
    /// - `aperture`: The simulated aperture of the camera lens
    /// - `focus_distance`: The distance from the lens to the focal point
    /// - `shutter_open_time`: Point of time at which the camera shutter opens
    /// - `shutter_close_time`: Point of time at which the camera shutter closes
    pub fn new(
        look_from: Vec3,
        look_at: Vec3,
        view_up: Vec3,
        vert_fov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_distance: f64,
        shutter_open_time: f64,
        shutter_close_time: f64,
    ) -> Self {
        let theta = vert_fov * std::f64::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect_ratio * half_height;
        let w = unit_vector(look_from - look_at);
        let u = unit_vector(cross(&view_up, &w));
        let v = cross(&w, &u);
        Camera {
            lens_radius: aperture / 2.0,
            lower_left_corner: look_from
                - (half_width * focus_distance * u)
                - (half_height * focus_distance * v)
                - (focus_distance * w),
            horizontal: 2.0 * half_width * focus_distance * u,
            vertical: 2.0 * half_height * focus_distance * v,
            origin: look_from,
            u,
            v,
            shutter_open_time,
            shutter_close_time,
        }
    }

    /// Creates a new Ray directed at the given coordinates, originating from the Camera's origin
    pub fn create_ray(&self, x: f64, y: f64) -> Ray {
        let rand_point = self.lens_radius * utils::random_point_in_unit_disk();
        let offset = (self.u * rand_point.x()) + (self.v * rand_point.y());
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + (x * self.horizontal) + (y * self.vertical)
                - self.origin
                - offset,
            self.shutter_open_time
                + rand::thread_rng().gen::<f64>()
                    * (self.shutter_close_time - self.shutter_open_time),
        )
    }
}
