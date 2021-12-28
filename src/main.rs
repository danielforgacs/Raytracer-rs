mod img;
mod vec3;
mod ray;
mod hittable;
mod hittable_list;
mod sphere;
mod camera;
mod material;

use img::*;
use vec3::*;
use ray::*;
use hittable::*;
use hittable_list::*;
use sphere::*;
use camera::*;

use rand::prelude::*;

const WIDTH: usize = 800;
const ASPECT_RATIO: f64 = 2.0 / 1.0;
const MAX_COLOUR_CALC_RECURSION: u32 = 10_000;

fn main() {
    let mut image = Image::new(
        WIDTH,
        ASPECT_RATIO
    );

    println!("image width x height:           {} x {}", image.width(), image.height());
    println!("aspect ratio:                   {}", image.aspect_ratio());
    println!("samples (rays / pixel):         {}", image.samples());
    println!("num pixels:                     {}", image.width() * image.height());
    let camera = Camera::new();
    println!("rendering...");
    render(&mut image, &camera);
    println!("writing image...");
    image.write();
    println!("finished.");
}

fn calculate_colour(ray: &Ray, world: &HittableList, col_cal_depth: u32) -> Colour {
    if col_cal_depth == MAX_COLOUR_CALC_RECURSION {
        return Colour::black();
    }

    let mut rec = HitRecord::default();
    let is_hit = world.hit(ray, &0.0, std::f64::MAX, &mut rec);

    if is_hit {
        let rand_sphere = random_in_unit_sphere();
        let target = rec.p + rec.normal + rand_sphere;
        let ray_inner = Ray::new(&rec.p, &(target - rec.p));
        /*
        THIS PART CAN OVERFLOW!
        Check the loop index.
        */
        return calculate_colour(&ray_inner, &world, col_cal_depth + 1) * 0.5;

    } else {
        let unit_direction = unit_vector(&ray.direction());
        let t = (unit_direction.y() + 1.0) * 0.5;
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
            - Vec3::new(0.0, 0.0, 0.0)
            * 2.0;

        if p.squared_length() < 1.0 {
            return p;
        }
    }
}

fn render(image: &mut Image, cam: &Camera) {
    let mut list: Vec<Box<dyn Hittable>> = Vec::new();
    list.push(Box::new(Sphere::new(Point3::new(-0.2, 0.2, -0.4), 0.1)));
    list.push(Box::new(Sphere::new(Point3::new(-0.5, 0.2, -0.6), 0.1)));
    list.push(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    list.push(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));
    let world = HittableList::new(list);
    let mut rng = rand::thread_rng();

    for y in (0..image.height()).rev() {
        if (image.height() - y) % 50 == 0 {
            println!("rendering line: {:04} / {}", image.height() - y, image.height());
        }
        let mut scan_line = Vec::new();
        for x in (0..image.width()).rev() {
            let mut colour = Colour::new(0.0, 0.0, 0.0);

            for _ in 0..image.samples() {
                let u_rand = (rng.gen::<f64>() * 2.0) - 1.0;
                let v_rand = (rng.gen::<f64>() * 2.0) - 1.0;
                let u = ((x as f64) + u_rand) / image.width() as f64;
                let v = ((y as f64) + v_rand) / image.height() as f64;
                let r = cam.get_ray(u, v);
                colour = colour + calculate_colour(&r, &world, 0);

            }

            colour = colour / image.samples() as f64;
            // gamma
            colour = Vec3::new(colour.x().sqrt(), colour.y().sqrt(), colour.z().sqrt());
            let rgb = [
                (colour.x() * 255.9) as u8,
                (colour.y() * 255.9) as u8,
                (colour.z() * 255.9) as u8,

            ];
            scan_line.push([rgb[0], rgb[1], rgb[2]]);
        }
        image.pixels.push(scan_line);
    }
}
