pub mod camera;
pub mod hittable;
pub mod image;
pub mod ray;

use glam::Vec3;
use rand::Rng;

use std::{
    fs::File,
    io::{self, Write},
};

use crate::{
    camera::Camera,
    hittable::{Hittable, Sphere, World},
    image::{Image, Pixel},
    ray::Ray,
};

fn ray_color(ray: &Ray, world: &World) -> Vec3 {
    match world.hit(ray, 0.0, f32::INFINITY) {
        Some(hit) => 0.5 * (hit.normal + Vec3::ONE),
        None => {
            let unit = ray.direction.normalize();
            let t = 0.5 * (unit.y + 1.0);
            (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
        }
    }
}

fn main() -> anyhow::Result<()> {
    let image_aspect = 16.0 / 9.0;
    let image_height = 400;
    let image_width = ((image_height as f32) * image_aspect) as usize;
    let samples_per_pixel = 100;

    let mut image = Image::new(image_width, image_height, Pixel::BLACK);

    let camera = Camera::new(&Default::default());

    let world = World {
        objects: vec![
            Box::new(Sphere {
                center: Vec3::new(0.0, 0.0, -1.0),
                radius: 0.5,
            }),
            Box::new(Sphere {
                center: Vec3::new(0.0, -100.5, -1.0),
                radius: 100.0,
            }),
        ],
    };

    let mut rng = rand::thread_rng();

    for y in 0..image_height {
        let up_y = image_height - 1 - y;
        eprint!("\r {} ...     ", up_y);
        io::stdout().flush()?;

        for x in 0..image_width {
            let mut sum = Vec3::ZERO;
            for _ in 0..samples_per_pixel {
                let du: f32 = rng.gen();
                let dv: f32 = rng.gen();

                let u = (x as f32 + du) / (image_width as f32);
                let v = (up_y as f32 + dv) / (image_height as f32);
                let ray = camera.get_ray(u, v);
                sum += ray_color(&ray, &world);
            }
            *image.pixel_mut(x, y) = (sum / (samples_per_pixel as f32)).into();
        }
    }
    eprintln!();

    image.write_ppm(File::create("test.ppm")?)?;
    Ok(())
}
