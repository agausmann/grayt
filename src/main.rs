pub mod camera;
pub mod hittable;
pub mod image;
pub mod material;
pub mod ray;

use glam::DVec3;
use rand::Rng;

use std::{
    fs::File,
    io::{self, Write},
};

use crate::{
    camera::Camera,
    hittable::{Hittable, Sphere, World},
    image::{Image, Pixel},
    material::{Dielectric, Lambertian, Metal},
    ray::Ray,
};

fn ray_color(ray: &Ray, world: &World, depth: u32) -> DVec3 {
    let mut ray = ray.clone();
    let mut color = DVec3::ONE;
    for _ in 0..depth {
        match world.hit(&ray, 0.001, f64::INFINITY) {
            Some(hit) => {
                if let Some(scatter) = hit.material.scatter(&ray, &hit) {
                    color *= scatter.attenuation;
                    ray = scatter.ray;
                } else {
                    break;
                }
            }
            None => {
                break;
            }
        }
    }
    let unit = ray.direction.normalize();
    let t = 0.5 * (unit.y + 1.0);
    let ambient = (1.0 - t) * DVec3::new(1.0, 1.0, 1.0) + t * DVec3::new(0.5, 0.7, 1.0);
    ambient * color
}

fn main() -> anyhow::Result<()> {
    let image_aspect = 16.0 / 9.0;
    let image_height = 400;
    let image_width = ((image_height as f64) * image_aspect) as usize;
    let samples_per_pixel = 100;
    let max_depth = 50;

    let ground = Lambertian {
        albedo: DVec3::new(0.8, 0.8, 0.0),
    };
    let center = Lambertian {
        albedo: DVec3::new(0.1, 0.2, 0.5),
    };
    let left = Dielectric { ir: 1.5 };
    let right = Metal {
        albedo: DVec3::new(0.8, 0.6, 0.2),
        fuzz: 0.0,
    };

    let mut image = Image::new(image_width, image_height, Pixel::BLACK);

    let camera = Camera::new(&Default::default());

    let world = World {
        objects: vec![
            Box::new(Sphere {
                center: DVec3::new(0.0, -100.5, -1.0),
                radius: 100.0,
                material: ground,
            }),
            Box::new(Sphere {
                center: DVec3::new(0.0, 0.0, -1.0),
                radius: 0.5,
                material: center,
            }),
            Box::new(Sphere {
                center: DVec3::new(-1.0, 0.0, -1.0),
                radius: 0.5,
                material: left.clone(),
            }),
            Box::new(Sphere {
                center: DVec3::new(-1.0, 0.0, -1.0),
                radius: -0.4,
                material: left,
            }),
            Box::new(Sphere {
                center: DVec3::new(1.0, 0.0, -1.0),
                radius: 0.5,
                material: right,
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
                sum += ray_color(&ray, &world, max_depth);
            }
            *image.pixel_mut(x, y) = (sum / (samples_per_pixel as f64)).powf(0.5).into();
        }
    }
    eprintln!();

    image.write_ppm(File::create("test.ppm")?)?;
    Ok(())
}
