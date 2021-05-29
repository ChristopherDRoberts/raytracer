use crate::linear_algebra::{Ray, Vec3};
use std::f64::consts::PI;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(look_from: Vec3, look_at: Vec3, vec_up: Vec3, vertical_fov : f64, aspect_ratio: f64) -> Self {
        let fov_in_radians = (vertical_fov / 180.0) * PI;
        let viewport_height = 2.0 * (fov_in_radians / 2.0).tan();
        let viewport_width = aspect_ratio * viewport_height;
        
        let w = (look_from - look_at).unit_vector();
        let u = (vec_up.cross(&w)).unit_vector();
        let v = w.cross(&u);

        let origin = look_from;
        let horizontal = u * viewport_width;
        let vertical = v * viewport_height;
        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - w;

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
}
