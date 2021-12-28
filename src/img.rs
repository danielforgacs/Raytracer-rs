use std::io::Write;
use std::fs::File;

pub struct Image {
    width: usize,
    aspect_ratio: f64,
    samples: u16,
    pub pixels: Vec<Vec<[u8; 3]>>,
}

impl Image {
    pub fn new(width: usize, aspect_ratio: f64) -> Self {
        Self {
            width,
            aspect_ratio,
            samples: 8,
            pixels: Vec::new(),
        }
    }

    pub fn write(&self) {
        let mut image_text = format!("P3\n{} {}\n255\n", self.width(), self.height());
        for y in &self.pixels {
            for x in y {
                image_text.push_str(
                    &format!("\n{} {} {}", x[0], x[1], x[2])
                )
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
        ((self.width() as f64) / self.aspect_ratio()) as usize
    }

    pub fn aspect_ratio(&self) -> f64 {
        self.aspect_ratio
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn samples(&self) -> u16 {
        self.samples
    }
}

pub fn _generate_test_image(image: &mut Image) {
    for y in 0..image.height() {
        let mut row: Vec<[u8; 3]> = Vec::new();
        for x in 0..image.width() {
            let r = ((((x as f64) + 1.0) / (image.width() as f64)) * 255.9) as u8;
            let g = ((((y as f64) + 1.0) / (image.width() as f64)) * 255.9) as u8;
            let b = 3;
            row.push([r, g, b]);
        }
        image.pixels.push(row);
    }
}
