use std::sync::Arc;

use glam::DVec3;

use crate::{
    camera::{Camera, CameraDescriptor},
    hittable::{Sphere, World},
    material::Lambertian,
    texture::Noise,
};

use super::Scene;

pub fn build(aspect_ratio: f64) -> Scene {
    let camera_desc = CameraDescriptor {
        origin: DVec3::new(13.0, 2.0, 3.0),
        look_at: DVec3::ZERO,
        vfov: 20.0,
        aspect_ratio,
        ..Default::default()
    };
    let camera = Camera::new(&camera_desc);

    let noise = Arc::new(Lambertian {
        albedo: Noise::new(&mut rand::thread_rng(), 4.0),
    });

    let mut world = World::new();
    world.add(Sphere {
        center: -1000.0 * DVec3::Y,
        radius: 1000.0,
        material: Arc::clone(&noise),
    });
    world.add(Sphere {
        center: 2.0 * DVec3::Y,
        radius: 2.0,
        material: Arc::clone(&noise),
    });

    Scene { world, camera }
}
