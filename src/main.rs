mod img;
mod vec3;
mod ray;
mod hittable;
mod hittable_list;
mod sphere;
mod camera;

use img::*;
use vec3::*;
use ray::*;
use hittable::*;
use hittable_list::*;
use sphere::*;
use camera::*;

use rand::prelude::*;

fn main() {
    let mut image = Image::new(800, 2.0 / 1.0);
    let camera = Camera::new();
    render(&mut image, &camera);
    image.write();
}

fn calculate_colour(r: &Ray, world: &HittableList) -> Colour {
    let mut rec = HitRecord::default();
    if world.hit(r, &0.0, std::f64::MAX, &mut rec) {
        return Vec3::new(rec.normal.x() + 1.0, rec.normal.y() + 1.0, rec.normal.z() + 1.0) * 0.5;
    } else {
        let unit_direction = unit_vector(&r.direction());
        let t = (unit_direction.y() + 1.0) * 0.5;
        return Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.3, 0.3, 1.0) * t;
    }

}

fn render(image: &mut Image, cam: &Camera) {
    let mut list: Vec<Box<dyn Hittable>> = Vec::new();
    list.push(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    list.push(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));
    let world = HittableList::new(list);
    let mut rng = rand::thread_rng();

    for y in (0..image.height()).rev() {
        let mut scan_line = Vec::new();
        for x in (0..image.width()).rev() {
            let mut colour = Colour::new(0.0, 0.0, 0.0);

            for _ in 0..image.samples() {
                let u_rand = (rng.gen::<f64>() * 2.0) - 1.0;
                let v_rand = (rng.gen::<f64>() * 2.0) - 1.0;
                let u = ((x as f64) + u_rand) / image.width() as f64;
                let v = ((y as f64) + v_rand) / image.height() as f64;
                let r = cam.get_ray(u, v);
                colour = colour + calculate_colour(&r, &world);

            }

            colour = colour / image.samples() as f64;
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
