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
use hittable::Hittable;
use hittable_list::HittableList;
use sphere::Sphere;
use camera::Camera;
use material::{Material, scatter};

use rand::prelude::*;

const WIDTH: usize = 400;
const RAY_PER_PIXEL_SAMPLES: u8 = 4;

const ASPECT_RATIO: f64 = 2.0 / 1.0;
const MAX_COLOUR_CALC_RECURSION: u32 = 256;

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
    image.gamma(1.0);
    println!("writing image...");
    image.write();
    println!("finished.");
}

fn calculate_colour(r: &Ray, world: &HittableList, col_cal_depth: u32) -> Colour {
    if col_cal_depth == MAX_COLOUR_CALC_RECURSION {
        println!("--> hit colour calc recursion limit");
        return Colour::black();
    }

    if let Some(rec) = world.hit(&r, 0.001, std::f64::MAX) {
        let mut scattered = Ray::new(Vec3::default(), Vec3::default());
        let mut attentuation = Vec3::default();

        if scatter(&rec.material, r, &rec, &mut attentuation, &mut scattered) {
            return attentuation * calculate_colour(&scattered, world, col_cal_depth + 1);
        } else {
            return Vec3::black();
        }
    } else {
        let unit_direction = unit_vector(&r.direction);
        let t = 0.5 * (unit_direction.y() + 1.0);
        return Vec3::white() * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t;
    }
}

fn random_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();

    loop {
        let p = Vec3::new(
            rng.gen::<f64>(),
            rng.gen::<f64>(),
            rng.gen::<f64>())
            * 2.0
            - Vec3::new(1.0, 1.0, 1.0);

        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

fn render(image: &mut Image, cam: &Camera) {
    let mut list: Vec<Box<dyn Hittable>> = Vec::new();
    list.push(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, Material::Lambert { albedo: Vec3::new(0.8, 0.8, 0.0)})));
    list.push(Box::new(Sphere::new(Point3::new(-0.23, 0.12, -0.5), 0.27, Material::Dielectric { refr_idx: 0.33})));
    list.push(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, Material::Lambert { albedo: Vec3::new(0.8, 0.3, 0.3)})));
    list.push(Box::new(Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, Material::Metal { albedo: Vec3::new(0.8, 0.6, 0.2), fuzz: 0.27})));
    list.push(Box::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, Material::Metal { albedo: Vec3::new(0.8, 0.8, 0.8), fuzz: 0.0})));
    let world = HittableList::new(list);
    let mut rng = rand::thread_rng();
    let notice_divisions = match (image.height() as f64 / 16.0) as usize {
        0 => 1,
        x => x
    };

    for y in (0..image.height()).rev() {
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
            let rgb = [colour.x(), colour.y(), colour.z()];
            scan_line.push(rgb);
        }
        image.pixels.push(scan_line);

        if (image.height() - y) % notice_divisions == 0 {
            let progress = 100.0 - ((y as f64 /  image.height() as f64) * 100.0);
            println!("> {:>8.1}%",  progress);
        }
    }
}
