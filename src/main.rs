pub mod image;

use std::fs::File;

use image::{Image, Pixel};

fn main() -> anyhow::Result<()> {
    let mut image = Image::new(200, 200, Pixel::BLACK);
    let fwidth = image.width() as f32;
    let fheight = image.height() as f32;

    for y in 0..image.height() {
        for x in 0..image.width() {
            let pixel = image.pixel_mut(x, y);
            pixel.r = (x as f32) / fwidth;
            pixel.g = (y as f32) / fheight;
            pixel.b = 0.25;
        }
    }

    image.write_ppm(File::create("test.ppm")?)?;
    Ok(())
}
