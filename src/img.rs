use std::io::Write;
use std::fs::File;

pub struct Image {
    pub width: usize,
    pub aspect_ratio: f64,
    pub pixels: Vec<Vec<[f64; 3]>>,
}

impl Image {
    pub fn new(width: usize, aspect_ratio: f64) -> Self {
        Self {
            width,
            aspect_ratio,
            pixels: Vec::new(),
        }
    }

    pub fn gamma(&mut self) {
        let gamma: f64 = 0.6;
        for y in 0..self.width {
            for x in 0..self.height() {
                self.pixels[x][y] = [
                    self.pixels[x][y][0].powf(1.0 / gamma),
                    self.pixels[x][y][1].powf(1.0 / gamma),
                    self.pixels[x][y][2].powf(1.0 / gamma),
                ];
            }
        }

    }

    pub fn write(&self) {
        let mut image_text = format!("P3\n{} {}\n255\n", self.width, self.height());

        for y in &self.pixels {
            for x in y {
                let (r, g, b) = (
                    (x[0] * 255.999) as u8,
                    (x[1] * 255.999) as u8,
                    (x[2] * 255.999) as u8
                );
                image_text.push_str(&format!("\n{} {} {}", r, g, b));
            };
        }

        let image_buf = image_text
            .chars()
            .map(|x| x as u8)
            .collect::<Vec<u8>>();

        let mut file = File::create("my_image.ppm").unwrap();
        file.write_all(&image_buf).unwrap();
    }

    pub fn height(&self) -> usize {
        ((self.width as f64) / self.aspect_ratio) as usize
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn generate_test_image() {
        let mut image = Image::new(24, 2.0);

        for y in 0..image.height() {
            let mut row: Vec<[f64; 3]> = Vec::new();
            for x in 0..image.width {
                let r = ((x as f64) + 1.0) / (image.width as f64);
                let g = ((y as f64) + 1.0) / (image.width as f64);
                let b = 0.1;
                row.push([r, g, b]);
            }
            image.pixels.push(row);
        }

        image.write();
    }
}
