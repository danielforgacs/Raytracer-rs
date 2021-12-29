use crate::vec3::{Vec3, dot, unit_vector};
use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::random_in_unit_sphere;

#[derive(Debug, Clone, Copy)]
pub enum Material {
    Lambert { albedo: Vec3 },
    Metal { albedo: Vec3, fuzz: f64 },
    Dielectric { refr_idx: f64 },
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
            *attenuation = albedo;
            *scattered = Ray::new(rec.p, target - rec.p);
            return true;
        }
        &Material::Metal { albedo, fuzz} => {
            let reflected = reflect(&unit_vector(&ray_in.direction), &rec.normal);
            if fuzz == 0.0 {
                *scattered = Ray::new(rec.p, reflected);
            } else {
                *scattered = Ray::new(rec.p, reflected + random_in_unit_sphere() * fuzz);
            }
            *attenuation = albedo;
            return dot(&scattered.direction, &rec.normal) > 0.0;
        }
        &Material::Dielectric { refr_idx} => {
            let reflected = reflect(&ray_in.direction, &rec.normal);
            // ??? let attenuation = Vec3::new(1.0, 1.0, 1.0);
            *attenuation = Vec3::new(1.0, 1.0, 1.0);
            let mut out_normal = Vec3::default();
            let mut refracted = Vec3::default();
            let mut ni_over_nt = 0.0;

            if dot(&ray_in.direction, &rec.normal) > 0.0 {
                out_normal = rec.normal * -1.0;
                ni_over_nt = refr_idx;
            } else {
                out_normal = rec.normal;
                ni_over_nt = 1.0 / refr_idx;
            }

            if refract(&ray_in.direction, &out_normal, ni_over_nt, &mut refracted) {
                *scattered = Ray::new(rec.p, refracted);
                return true;
            } else {
                *scattered = Ray::new(rec.p, reflected);
                return false;

            }
        }
    }
    // return false;
}

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - (*n * dot(v, n) * 2.0)
}

pub fn refract(v: &Vec3, n: &Vec3, ni_over_ft: f64, refracted: &mut Vec3) -> bool {
    let uv = unit_vector(v);
    let dt = dot(&uv, n);
    let discriminant = 1.0 - ni_over_ft * ni_over_ft * (1.0 - dt * dt);
    if discriminant > 0.0 {
        *refracted = ((uv - *n * dt) * ni_over_ft) - *n * discriminant.sqrt();
        true
    } else {
        false
    }
}
