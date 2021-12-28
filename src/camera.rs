use crate::vec3::*;
use crate::ray::*;

pub struct Camera {
    lower_left_corner: Vec3,
    horizontal: Point3,
    vertical: Point3,
    origin: Vec3,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            lower_left_corner: Point3::new(-2.0, -1.0, -1.0),
            horizontal: Vec3::new(4.0, 0.0, 0.0),
            vertical: Vec3::new(0.0, 2.0, 0.0),
            origin: Point3::new(0.0, 0.0, 0.0),
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            &self.origin,
            &(self.lower_left_corner
            + self.horizontal * u
            + self.vertical * v
            - self.origin)
        )
    }
}
