pub mod camera;
pub mod hittable;
pub mod image;
pub mod material;
pub mod perlin;
pub mod ray;
pub mod scene;
pub mod texture;

use glam::DVec3;
use rand::Rng;

use std::{
    f64::consts as f64,
    fs::File,
    io::{self, Write},
};

use crate::{
    hittable::{Hittable, World},
    image::{Image, Pixel},
    ray::Ray,
    scene::Scene,
};

fn main() -> anyhow::Result<()> {
    let image_aspect = 16.0 / 9.0;
    let image_height = 400;
    let image_width = ((image_height as f64) * image_aspect) as usize;
    let samples_per_pixel = 400;
    let max_depth = 50;

    let mut image = Image::new(image_width, image_height, Pixel::BLACK);

    let Scene {
        world,
        camera,
        background,
    } = scene::simple_light::build(image_aspect);

    let mut rng = rand::thread_rng();

    for y in 0..image_height {
        let up_y = image_height - 1 - y;
        eprint!("\r {} ...     ", up_y);
        io::stdout().flush()?;

        for x in 0..image_width {
            let mut sum = DVec3::ZERO;
            for _ in 0..samples_per_pixel {
                let du: f64 = rng.gen();
                let dv: f64 = rng.gen();

                let u = (x as f64 + du) / (image_width as f64);
                let v = (up_y as f64 + dv) / (image_height as f64);
                let ray = camera.get_ray(u, v);
                sum += ray_color(&ray, background, &world, max_depth);
            }
            *image.pixel_mut(x, y) = (sum / (samples_per_pixel as f64)).powf(0.5).into();
        }
    }
    eprintln!();

    image.write_ppm(File::create("test.ppm")?)?;
    Ok(())
}

fn ray_color(ray: &Ray, background: DVec3, world: &World, depth: u32) -> DVec3 {
    let mut ray = ray.clone();
    let mut total = DVec3::ZERO;
    let mut atten = DVec3::ONE;
    for _ in 0..depth {
        let hit = match world.hit(&ray, 0.001, f64::INFINITY) {
            Some(x) => x,
            None => {
                total += atten * background;
                return total;
            }
        };

        total += atten * hit.material.emitted(hit.uv, hit.point);

        match hit.material.scatter(&ray, &hit) {
            Some(scatter) => {
                atten *= scatter.attenuation;
                ray = scatter.ray;
            }
            None => {
                total += atten * background;
                return total;
            }
        }
    }
    DVec3::ZERO
}
