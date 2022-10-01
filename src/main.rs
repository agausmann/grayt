pub mod camera;
pub mod hittable;
pub mod image;
pub mod material;
pub mod ray;

use glam::DVec3;
use rand::Rng;

use std::{
    f64::consts as f64,
    fs::File,
    io::{self, Write},
};

use crate::{
    camera::{Camera, CameraDescriptor},
    hittable::{Hittable, Sphere, World},
    image::{Image, Pixel},
    material::{Dielectric, Lambertian, Metal},
    ray::Ray,
};

fn ray_color(ray: &Ray, world: &World, depth: u32) -> DVec3 {
    let mut ray = ray.clone();
    let mut color = DVec3::ONE;
    for _ in 0..depth {
        let maybe_scatter = world
            .hit(&ray, 0.001, f64::INFINITY)
            .and_then(|hit| hit.material.scatter(&ray, &hit));

        match maybe_scatter {
            Some(scatter) => {
                color *= scatter.attenuation;
                ray = scatter.ray;
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

fn ch10() -> World {
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

    let mut world = World::new();
    world.add(Sphere {
        center: DVec3::new(0.0, -100.5, -1.0),
        radius: 100.0,
        material: ground,
    });
    world.add(Sphere {
        center: DVec3::new(0.0, 0.0, -1.0),
        radius: 0.5,
        material: center,
    });
    world.add(Sphere {
        center: DVec3::new(-1.0, 0.0, -1.0),
        radius: 0.5,
        material: left.clone(),
    });
    world.add(Sphere {
        center: DVec3::new(-1.0, 0.0, -1.0),
        radius: -0.4,
        material: left,
    });
    world.add(Sphere {
        center: DVec3::new(1.0, 0.0, -1.0),
        radius: 0.5,
        material: right,
    });
    world
}

fn ch11() -> World {
    let r = (f64::PI / 4.0).cos();
    let left = Lambertian {
        albedo: DVec3::new(0.0, 0.0, 1.0),
    };
    let right = Lambertian {
        albedo: DVec3::new(1.0, 0.0, 0.0),
    };

    let mut world = World::new();
    world.add(Sphere {
        center: DVec3::new(-r, 0.0, -1.0),
        radius: r,
        material: left,
    });
    world.add(Sphere {
        center: DVec3::new(r, 0.0, -1.0),
        radius: r,
        material: right,
    });
    world
}

fn ch13() -> World {
    let mut world = World::new();

    world.add(Sphere {
        center: DVec3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Lambertian {
            albedo: DVec3::new(0.5, 0.5, 0.5),
        },
    });

    let mut rng = rand::thread_rng();
    let keepout_center = DVec3::new(4.0, 0.2, 0.0);

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen_range(0.0..1.0);
            let center = DVec3::new(
                a as f64 + 0.9 * rng.gen_range(0.0..1.0),
                0.2,
                b as f64 + 0.9 * rng.gen_range(0.0..1.0),
            );
            let radius = 0.2;
            if center.distance(keepout_center) <= 0.9 {
                continue;
            }
            if choose_mat < 0.8 {
                let material = Lambertian {
                    albedo: DVec3::new(
                        rng.gen_range(0.0..1.0) * rng.gen_range(0.0..1.0),
                        rng.gen_range(0.0..1.0) * rng.gen_range(0.0..1.0),
                        rng.gen_range(0.0..1.0) * rng.gen_range(0.0..1.0),
                    ),
                };
                world.add(Sphere {
                    center,
                    radius,
                    material,
                });
            } else if choose_mat < 0.95 {
                let material = Metal {
                    albedo: DVec3::new(
                        rng.gen_range(0.5..1.0),
                        rng.gen_range(0.5..1.0),
                        rng.gen_range(0.5..1.0),
                    ),
                    fuzz: rng.gen_range(0.0..0.5),
                };
                world.add(Sphere {
                    center,
                    radius,
                    material,
                });
            } else {
                let material = Dielectric { ir: 1.5 };
                world.add(Sphere {
                    center,
                    radius,
                    material,
                });
            }
        }
    }

    world.add(Sphere {
        center: DVec3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        material: Dielectric { ir: 1.5 },
    });
    world.add(Sphere {
        center: DVec3::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: Lambertian {
            albedo: DVec3::new(0.4, 0.2, 0.1),
        },
    });
    world.add(Sphere {
        center: DVec3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material: Metal {
            albedo: DVec3::new(0.7, 0.6, 0.5),
            fuzz: 0.0,
        },
    });

    world
}

fn main() -> anyhow::Result<()> {
    let image_aspect = 3.0 / 2.0;
    let image_height = 800;
    let image_width = ((image_height as f64) * image_aspect) as usize;
    let samples_per_pixel = 500;
    let max_depth = 50;

    let mut image = Image::new(image_width, image_height, Pixel::BLACK);

    let camera = Camera::new(&CameraDescriptor {
        origin: DVec3::new(13.0, 2.0, 3.0),
        look_at: DVec3::ZERO,
        vfov: 20.0,
        aspect_ratio: image_aspect,
        aperture: 0.1,
        focus_distance: Some(10.0),
        ..Default::default()
    });

    let world = ch13();

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
