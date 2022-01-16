mod image;
mod vec3;
mod ray;
mod hitrecord;

use vec3::{Point3, Vec3};

fn main() {
    let mut image = image::Image::new()
        .set_width(400)
        .set_aspect_ratio(16.0 / 9.0);
    println!("{:<30}{}px x {}px", "Image dimensions:", image.get_width(), image.get_height());
    println!("{:<30}{}", "apsect ratio:", image.get_aspect_ratio());

    let viewport_h: f64 = 2.0;
    let viewport_w = image.get_aspect_ratio() * viewport_h;
    let focal_length = 1.0;

    let origin = Point3::new();
    let horizontal = Vec3::new()
        .set_xyz(viewport_w, 0.0, 0.0);
    let vertical = Vec3::new()
        .set_xyz(0.0, viewport_h, 0.0);

    let lower_left_corner = origin
        - (horizontal / 2.0)
        - (vertical / 2.0)
        - Vec3::new().set_xyz(0.0, 0.0, focal_length);

    for y in (0..image.get_height()).rev() {
        for x in 0..image.get_width() {
            let u = x as f64 / (image.get_width() - 1) as f64;
            let v = y as f64 / (image.get_height() - 1) as f64;
            let ray = ray::Ray::new(
                origin,
                lower_left_corner + (u * horizontal) + (v * vertical) - origin,
            );
            let colour = ray::ray_colour(&ray);
            image.set_pixel(
                colour.get_x(),
                colour.get_y(),
                colour.get_z(),
                x,
                y,
            );

        }
    }


    image.save_ppm();
}
