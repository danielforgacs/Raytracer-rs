use crate::vec3::{Point3, Vec3, Colour, dot, unit_vector};

pub struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self {
            origin,
            direction,
        }
    }

    pub fn get_origin(&self) -> Vec3 {
        self.origin
    }

    pub fn get_direction(&self) -> Vec3 {
        self.direction
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.get_direction()
    }
}

pub fn ray_colour(ray: &Ray) -> Colour {
    let t = hit_sphere(
        &Point3::new().set_xyz(0.0, 0.0, -1.0),
        0.5,
        &ray,
    );
    if t > 0.0 {
        let normal = unit_vector(
            &(ray.at(t) - Vec3::new().set_xyz(0.0, 0.0, -1.0))
        );
        return 0.5 * Colour::new().set_xyz(
            normal.get_x() + 1.0,
            normal.get_y() + 1.0,
            normal.get_z() + 1.0,
        );
    }
    let unit_direction = unit_vector(&ray.get_direction());
    let t = 0.5 * (unit_direction.get_y() + 1.0);
    (1.0 - t)
        * Colour::new().white()
        + (t * Colour::new().set_xyz(0.5, 0.7, 1.0))
}

pub fn hit_sphere(center: &Point3, radius: f64, ray: &Ray) -> f64 {
    let oc = ray.get_origin() - *center;
    let a = ray.get_direction().length_squared();
    let half_b = dot(&oc, &ray.get_direction());
    let c = oc.length_squared() - radius.powi(2);
    let discriminant = half_b.powi(2) - a * c;
    if discriminant < 0.0 {
        return -1.0
    } else {
        return (-half_b - discriminant.sqrt()) / a;
    }
}
