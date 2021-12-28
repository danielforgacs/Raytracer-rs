mod img;
mod vec3;
mod ray;
mod hittable;
mod hittable_list;
mod sphere;

use img::*;
use vec3::*;
use ray::*;
use hittable::*;
use hittable_list::*;
use sphere::*;

fn main() {
    let mut image = Image::new(800, 2.0 / 1.0);
    render(&mut image);
    image.write();
}

fn colour(r: &Ray, world: &HittableList) -> Colour {
    let mut rec = HitRecord::default();
    if world.hit(r, &0.0, &std::f64::MAX, &mut rec) {
        return Vec3::new(rec.normal.x() + 1.0, rec.normal.y() + 1.0, rec.normal.z() + 1.0) * 0.5;
    } else {
        let unit_direction = unit_vector(&r.direction());
        let t = (unit_direction.y() + 1.0) * 0.5;
        return Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.3, 0.3, 1.0) * t;
    }

}

// fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> f64 {
//     let oc = r.origin() - *center;
//     let a = dot(&r.direction(), &r.direction());
//     let b = dot(&oc, &r.direction()) * 2.0;
//     let c = dot(&oc, &oc) - radius * radius;
//     let discriminant = b * b - a * c * 4.0;
//     if discriminant < 0.0 {
//         -1.0
//     } else {
//         (-b - discriminant.sqrt()) / (a * 2.0)
//     }
// }

fn render(image: &mut Image) {
    let lower_left_corner = Point3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Point3::new(0.0, 0.0, 0.0);

    let mut list: Vec<Box<dyn Hittable>> = Vec::new();
    list.push(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    list.push(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));
    let world = HittableList::new(list);

    for y in (0..image.height()).rev() {
        let mut scan_line = Vec::new();
        for x in (0..image.width()).rev() {
            let u = x as f64 / image.width() as f64;
            let v = y as f64 / image.height() as f64;
            let r = Ray::new(&origin, &(lower_left_corner + horizontal * u + vertical * v));
            let rgb = colour(&r, &world);
            let rgb = [
                (rgb.x() * 255.9) as u8,
                (rgb.y() * 255.9) as u8,
                (rgb.z() * 255.9) as u8,

            ];
            scan_line.push([rgb[0], rgb[1], rgb[2]]);
        }
        image.pixels.push(scan_line);
    }
}
