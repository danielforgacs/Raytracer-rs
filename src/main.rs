mod image;
mod vec3;
mod ray;

fn main() {
    let mut image = image::Image::new(256, 64, None);
    image.save_ppm();
}
