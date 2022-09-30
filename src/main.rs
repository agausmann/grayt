pub mod camera;
pub mod hittable;
pub mod image;
pub mod ray;

use glam::DVec3;
use rand::{distributions::Uniform, prelude::Distribution, Rng};

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

fn random_in_unit_sphere<R: Rng>(rng: &mut R) -> DVec3 {
    let dist = Uniform::new_inclusive(-1.0, 1.0);
    loop {
        let candidate = DVec3::new(dist.sample(rng), dist.sample(rng), dist.sample(rng));
        if candidate.length_squared() <= 1.0 {
            return candidate;
        }
    }
}

fn random_on_unit_sphere<R: Rng>(rng: &mut R) -> DVec3 {
    loop {
        let candidate = random_in_unit_sphere(rng);
        if let Some(unit) = candidate.try_normalize() {
            return unit;
        }
    }
}

fn random_on_hemisphere<R: Rng>(rng: &mut R, normal: DVec3) -> DVec3 {
    let unit = random_on_unit_sphere(rng);
    if unit.dot(normal) > 0.0 {
        unit
    } else {
        -unit
    }
}

fn ray_color<R: Rng>(ray: &Ray, world: &World, rng: &mut R, depth: u32) -> DVec3 {
    if depth == 0 {
        return DVec3::ZERO;
    }

    match world.hit(ray, 0.0, f64::INFINITY) {
        Some(hit) => {
            0.5 * ray_color(
                &Ray {
                    origin: hit.point,
                    direction: random_on_hemisphere(rng, hit.normal),
                },
                world,
                rng,
                depth - 1,
            )
        }
        None => {
            let unit = ray.direction.normalize();
            let t = 0.5 * (unit.y + 1.0);
            (1.0 - t) * DVec3::new(1.0, 1.0, 1.0) + t * DVec3::new(0.5, 0.7, 1.0)
        }
    }
}

fn main() -> anyhow::Result<()> {
    let image_aspect = 16.0 / 9.0;
    let image_height = 400;
    let image_width = ((image_height as f64) * image_aspect) as usize;
    let samples_per_pixel = 100;
    let max_depth = 50;

    let mut image = Image::new(image_width, image_height, Pixel::BLACK);

    let camera = Camera::new(&Default::default());

    let world = World {
        objects: vec![
            Box::new(Sphere {
                center: DVec3::new(0.0, 0.0, -1.0),
                radius: 0.5,
            }),
            Box::new(Sphere {
                center: DVec3::new(0.0, -100.5, -1.0),
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
            let mut sum = DVec3::ZERO;
            for _ in 0..samples_per_pixel {
                let du: f64 = rng.gen();
                let dv: f64 = rng.gen();

                let u = (x as f64 + du) / (image_width as f64);
                let v = (up_y as f64 + dv) / (image_height as f64);
                let ray = camera.get_ray(u, v);
                sum += ray_color(&ray, &world, &mut rng, max_depth);
            }
            *image.pixel_mut(x, y) = (sum / (samples_per_pixel as f64)).powf(0.5).into();
        }
    }
    eprintln!();

    image.write_ppm(File::create("test.ppm")?)?;
    Ok(())
}
