mod img;
mod vec3;
mod ray;
mod hittable;
mod hittable_list;
mod sphere;
mod camera;
mod material;

use img::Image;
use vec3::{Vec3, Colour, Point3, unit_vector};
use ray::Ray;
use hittable::{HitRecord, Hittable};
use hittable_list::HittableList;
use sphere::Sphere;
use camera::Camera;
use material::{Material, scatter};

use rand::prelude::*;

const WIDTH: usize = 200;
const RAY_PER_PIXEL_SAMPLES: u8 = 4;

const ASPECT_RATIO: f64 = 2.0 / 1.0;
const MAX_COLOUR_CALC_RECURSION: u32 = 100;

fn main() {
    let mut image = Image::new(
        WIDTH,
        ASPECT_RATIO
    );
    let camera = Camera::new(RAY_PER_PIXEL_SAMPLES);

    println!("image width x height:           {} x {}", image.width, image.height());
    println!("aspect ratio:                   {}", image.aspect_ratio);
    println!("samples (rays / pixel):         {}", camera.ray_p_pixel_samples);
    println!("num pixels:                     {}", image.width * image.height());
    println!("rendering...");
    render(&mut image, &camera);
    image.gamma(1.23);
    println!("writing image...");
    image.write();
    println!("finished.");
}

fn calculate_colour(r: &Ray, world: &HittableList, col_cal_depth: u32) -> Vec3 {
    if col_cal_depth == MAX_COLOUR_CALC_RECURSION {
        return Colour::black();
    }

    if let Some(rec) = world.hit(&r, &0.001, std::f64::MAX) {
        let mut scattered = Ray::new(&Vec3::default(), &Vec3::default());
        let mut attentuation = Vec3::default();

        if scatter(&rec.material, r, &rec, &mut attentuation, &mut scattered) {
            let cc = calculate_colour(&scattered, world, col_cal_depth + 1);
            let att = attentuation;
            return att * cc;
        } else {
            return Vec3::new(0.0, 0.0, 0.0);
        }
    } else {
        let unit_direction = unit_vector(&r.direction);
        let t = 0.5 * (unit_direction.y() + 1.0);

        Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
    }


//     // let mut rec = HitRecord::default();
//
//     match world.hit(&ray, &0.001, std::f64::MAX) {
//         Some(rec) => {
//             let mut scattered = Ray::new(&Vec3::new(0.0, 0.0, 0.0), &Vec3::new(1.0, 0.0, 0.0));
//             // println!("calc col::scatt: {:?}", scattered);
//             // let mut attenuation = Vec3::new(0.2, 0.2, 0.2);
//             let mut attenuation = Colour::new(0.2, 0.2, 0.2);
//             if scatter(&rec.material, ray, &rec, &mut attenuation, &mut scattered) {
//                 // let xyz: () = attenuation;
//                 // let xyz: () = calculate_colour(&ray, &world, col_cal_depth + 1);
//                 // return attenuation * calculate_colour(&ray, &world, col_cal_depth + 1);
//                 return calculate_colour(&ray, &world, col_cal_depth + 1) * 0.2;
//                 // return calculate_colour(&ray, &world, col_cal_depth + 1);
//                 // return attenuation;
//             } else {
//                 return Vec3::black();
//             }
//             // let target = rec.p + rec.normal + random_in_unit_sphere();
//             // let ray_inner = Ray::new(&rec.p, &(target - rec.p));
//             /*
//             THIS PART CAN OVERFLOW!
//             Check the loop index: col_cal_depth.
//             */
//             // return calculate_colour(&ray_inner, &world, col_cal_depth + 1) * 0.5;k
//         }
//         None => {
//             let unit_direction = unit_vector(&ray.direction);
//             let t = (unit_direction.y() + 1.0) * 0.5;
//             return Vec3::white() * (1.0 - t) + Vec3::new(0.12, 0.1, 0.77) * t;
//         }
//     }
}

fn random_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();

    loop {
        let p = Vec3::new(
            rng.gen::<f64>(),
            rng.gen::<f64>(),
            rng.gen::<f64>())
            - Vec3::new(0.0, 0.0, 0.0)
            * 2.0;

        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

fn render(image: &mut Image, cam: &Camera) {
    let mut list: Vec<Box<dyn Hittable>> = Vec::new();
    list.push(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, Material::Lambert { albedo: Vec3::new(0.8, 0.3, 0.3)})));
    list.push(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, Material::Lambert { albedo: Vec3::new(0.8, 0.3, 0.0)})));
    list.push(Box::new(Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, Material::Metal { albedo: Vec3::new(0.8, 0.6, 0.2)})));
    list.push(Box::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, Material::Metal { albedo: Vec3::new(0.8, 0.6, 0.8)})));
    let world = HittableList::new(list);
    let mut rng = rand::thread_rng();

    for y in (0..image.height()).rev() {
        if (image.height() - y) % 50 == 0 {
            println!("rendering line: {:04} / {}", image.height() - y, image.height());
        }
        let mut scan_line = Vec::new();

        for x in (0..image.width).rev() {
            let mut colour = Colour::white();

            for _ in 0..cam.ray_p_pixel_samples {
                let u_rand = (rng.gen::<f64>() * 2.0) - 1.0;
                let v_rand = (rng.gen::<f64>() * 2.0) - 1.0;
                let u = ((x as f64) + u_rand) / image.width as f64;
                let v = ((y as f64) + v_rand) / image.height() as f64;
                let r = cam.get_ray(u, v);
                colour = colour + calculate_colour(&r, &world, 0);
            }

            colour = colour / cam.ray_p_pixel_samples as f64;
            // Gamma:
            let rgb = [
                colour.x(),
                colour.y(),
                colour.z(),

            ];
            scan_line.push(rgb);
        }
        image.pixels.push(scan_line);
    }
}
