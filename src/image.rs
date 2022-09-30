use std::io::{self, Write};

use glam::DVec3;

#[derive(Debug, Clone, Copy)]
pub struct Pixel {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl From<[f64; 3]> for Pixel {
    fn from([r, g, b]: [f64; 3]) -> Self {
        Self { r, g, b }
    }
}

impl From<DVec3> for Pixel {
    fn from(vec: DVec3) -> Self {
        Pixel::rgb(vec.x, vec.y, vec.z)
    }
}

impl From<Pixel> for [f64; 3] {
    fn from(pixel: Pixel) -> Self {
        [pixel.r, pixel.g, pixel.b]
    }
}

impl From<Pixel> for DVec3 {
    fn from(pixel: Pixel) -> Self {
        DVec3::new(pixel.r, pixel.g, pixel.b)
    }
}

impl Pixel {
    pub const BLACK: Self = Self::rgb(0.0, 0.0, 0.0);

    pub const fn rgb(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }
}

#[derive(Clone)]
pub struct Image {
    pixels: Box<[Pixel]>,
    width: usize,
    height: usize,
}

impl Image {
    pub fn new(width: usize, height: usize, fill: Pixel) -> Self {
        Self {
            pixels: vec![fill; width * height].into_boxed_slice(),
            width,
            height,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn pixel(&self, x: usize, y: usize) -> &Pixel {
        assert!(x < self.width && y < self.height);
        &self.pixels[y * self.width + x]
    }

    pub fn pixel_mut(&mut self, x: usize, y: usize) -> &mut Pixel {
        assert!(x < self.width && y < self.height);
        &mut self.pixels[y * self.width + x]
    }

    pub fn write_ppm<W: Write>(&self, mut writer: W) -> io::Result<()> {
        let resolution = 255u8;
        let fres = resolution as f64;

        let mut buffer = Vec::with_capacity(self.width() * self.height() * 3);
        for y in 0..self.height() {
            for x in 0..self.width() {
                let pixel = self.pixel(x, y);
                for channel in [pixel.r, pixel.g, pixel.b] {
                    let clipped = (channel.clamp(0.0, 1.0) * fres).round() as u8;
                    buffer.push(clipped);
                }
            }
        }
        write!(
            writer,
            "P6 {} {} {} ",
            self.width(),
            self.height(),
            resolution
        )?;
        writer.write_all(&buffer)?;
        Ok(())
    }
}
