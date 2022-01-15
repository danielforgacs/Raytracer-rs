mod image;
mod vec3;
mod ray;

fn main() {
    let mut image = image::Image::new()
        .set_width(400)
        .set_height(225);

    let viewport_h = 2.0;
    let viewport_w = image.aspect_ratio() * viewport_h;
    let focal_length = 1.0;

    let origin = vec3::Point3::new(0.0, 0.0, 0.0);
    let horizontal = vec3::Vec3::new(viewport_w, 0.0, 0.0);
    let vertical = vec3::Vec3::new(0.0, viewport_h, 0.0);

    let lower_left_corner = origin
        - (horizontal / 2.0)
        - (vertical / 2.0)
        - vec3::Vec3::new(0.0, 0.0, focal_length);

    for y in 0..image.get_height() {
        for x in 0..image.get_width() {
            let u = x as f32 / (image.get_width() - 1) as f32;
            let v = x as f32 / (image.get_height() - 1) as f32;
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
