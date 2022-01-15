mod image;

fn main() {
    let mut image = image::Image::new(256, 64, None);
    for y in 0..image.get_height() {
        for x in 0..image.get_width() {
            let r = x as f32 / (image.get_width() - 1) as f32;
            let b = y as f32 / (image.get_height() - 1) as f32;
            println!("{:<15} {}", r, b);
            image.set_pixel(r, 0.0, b, y, x);
        }
    }
    image.save_ppm();
}
