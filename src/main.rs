mod img;
mod vec3;
mod ray;

use img::*;
use vec3::*;
use ray::*;

fn main() {
    let mut image = Image::new(800, 2.0 / 1.0);
    render(&mut image);
    image.write();
}

fn colour(r: &Ray) -> Colour {
    if hit_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5, &r) {
        return Colour::new(1.0, 0.0, 0.0);
    }
    let unit_direction = unit_vector(&r.direction());
    let t = (unit_direction.y() + 1.0) * 0.5;
    let colour = Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.3, 0.3, 1.0) * t;
    colour
}

fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> bool {
    let oc = r.origin() - *center;
    let a = dot(&r.direction(), &r.direction());
    let b = dot(&oc, &r.direction()) * 2.0;
    let c = dot(&oc, &oc) - radius * radius;
    let discriminant = b * b - a * c * 4.0;
    discriminant > 0.0
}

fn render(image: &mut Image) {
    let lower_left_corner = Point3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Point3::new(0.0, 0.0, 0.0);

    for y in (0..image.height()).rev() {
        let mut scan_line = Vec::new();
        for x in (0..image.width()).rev() {
            let u = x as f64 / image.width() as f64;
            let v = y as f64 / image.height() as f64;
            let r = Ray::new(&origin, &(lower_left_corner + horizontal * u + vertical * v));
            let rgb = colour(&r);
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
