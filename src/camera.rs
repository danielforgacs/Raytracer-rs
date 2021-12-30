use crate::vec3::{Vec3, Point3};
use crate::ray::Ray;

pub struct Camera {
    lower_left_corner: Vec3,
    horizontal: Point3,
    vertical: Point3,
    origin: Vec3,
    pub ray_p_pixel_samples: u8,
}

impl Camera {
    pub fn new(rpp_samples: u8, vfov: f64, aspect: f64) -> Self {
        let theta = vfov * std::f64::consts::PI / 180.0;
        let half_h = (theta / 2.0).tan();
        let half_w = aspect * half_h;
        Self {
            lower_left_corner: Point3::new(half_w * -1.0, half_h * -1.0, -1.0),
            horizontal: Vec3::new(half_w * 2.0, 0.0, 0.0),
            vertical: Vec3::new(0.0, half_h * 2.0, 0.0),
            origin: Point3::new(0.0, 0.0, 0.0),
            ray_p_pixel_samples: rpp_samples,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner
            + self.horizontal * u
            + self.vertical * v
            - self.origin
        )
    }
}
