use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::random_in_unit_sphere;

#[derive(Debug)]
pub enum Material {
    Lambert { albedo: Vec3 },
    Metal {},
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
) -> bool
{
    match material {
        &Material::Lambert { albedo} => {
            let target = rec.p + rec.normal + random_in_unit_sphere();
            /*
            WHICH WAY IS BETTER FOT THE NEXT TWO LINES?
            */
            // *attenuation = albedo;
            // *scattered = Ray::new(&rec.p, &(target - rec.p));
            attenuation = &mut albedo;
            scattered = &mut Ray::new(&rec.p, &(target - rec.p));
        }
        &Material::Metal {} => {}
        &Material::Dielectric {} => {}
    }
    false
}
