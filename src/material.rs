use crate::vec3::{Vec3, dot, unit_vector};
use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::random_in_unit_sphere;

#[derive(Debug, Clone, Copy)]
pub enum Material {
    Lambert { albedo: Vec3 },
    Metal { albedo: Vec3 },
    Dielectric {},
}

impl Default for Material {
    fn default() -> Self {
        Self::Lambert { albedo: Vec3::white() }
    }
}

pub fn scatter(
    material: &Material,
    ray_in: &Ray,
    rec: &HitRecord,
    attenuation: &mut Vec3,
    scattered: &mut Ray
) -> bool {
    match material {
        &Material::Lambert { albedo} => {
            let target = rec.p + rec.normal + random_in_unit_sphere();
            /*
            WHICH WAY IS BETTER FOT THE NEXT TWO LINES?
            */
            *attenuation = albedo;
            *scattered = Ray::new(&rec.p, &(target - rec.p));
            // attenuation = &mut albedo;
            // scattered = &mut Ray::new(&rec.p, &(target - rec.p));
            return true;
        }
        &Material::Metal { albedo} => {
            let reflected = reflect(&unit_vector(&ray_in.direction), &rec.normal);
            *scattered = Ray::new(&rec.p, &reflected);
            *attenuation = albedo;
            return dot(&scattered.direction, &rec.normal) > 0.0;
        }
        &Material::Dielectric {} => {
            return false;
        }
    }
}

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    // println!("refl: {:?}", *v - (*n * dot(v, n) * 2.0));
    *v - (*n * dot(v, n) * 2.0)
}
