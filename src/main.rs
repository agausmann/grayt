pub mod image;
pub mod ray;

use glam::Vec3;

use std::{
    fs::File,
    io::{self, Write},
};

use crate::image::{Image, Pixel};
use crate::ray::Ray;

fn ray_color(ray: &Ray) -> Vec3 {
    let unit = ray.direction.normalize();
    let t = 0.5 * (unit.y + 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}

fn main() -> anyhow::Result<()> {
    let image_aspect = 16.0 / 9.0;
    let image_height = 400;
    let image_width = ((image_height as f32) * image_aspect) as usize;

    let viewport_height = 2.0;
    let viewport_width = viewport_height * image_aspect;
    let focal_length = 1.0;

    let eye = Vec3::ZERO;
    let horizontal = viewport_width * Vec3::X;
    let vertical = viewport_height * Vec3::Y;
    let lower_left = eye - horizontal / 2.0 - vertical / 2.0 - focal_length * Vec3::Z;

    let mut image = Image::new(image_width, image_height, Pixel::BLACK);

    for y in 0..image_height {
        let up_y = image_height - 1 - y;
        eprint!("\r {} ...     ", up_y);
        io::stdout().flush()?;

        for x in 0..image_width {
            let u = (x as f32) / (image_width as f32);
            let v = (up_y as f32) / (image_height as f32);
            let ray = Ray {
                origin: eye,
                direction: lower_left + u * horizontal + v * vertical - eye,
            };

            let pixel = image.pixel_mut(x, y);
            *pixel = ray_color(&ray).into();
        }
    }
    eprintln!();

    image.write_ppm(File::create("test.ppm")?)?;
    Ok(())
}
