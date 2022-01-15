use crate::vec3;

pub struct Ray {
    origin: vec3::Point3,
    direction: vec3::Vec3,
}

impl Ray {
    pub fn new(origin: vec3::Point3, direction: vec3::Vec3) -> Self {
        Self {
            origin,
            direction,
        }
    }

    fn get_direction(&self) -> vec3::Vec3 {
        self.direction
    }

    fn at(&self, t: f64) -> vec3::Point3 {
        self.origin + t * self.get_direction()
    }
}

pub fn ray_colour(ray: &Ray) -> vec3::Colour {
    let unit_direction = vec3::unit_vector(&ray.get_direction());
    let t = 0.5 * (unit_direction.get_y() + 1.0);
    (1.0 - t)
        * vec3::Colour::new(1.0, 1.0, 1.0)
        + (t * vec3::Colour::new(0.5, 0.7, 1.0))
}
