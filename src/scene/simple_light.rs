use std::sync::Arc;

use glam::{DVec2, DVec3};

use crate::{
    camera::{Camera, CameraDescriptor},
    hittable::{Sphere, World, XYRect},
    material::{DiffuseLight, Lambertian},
    texture::{Noise, Solid},
};

use super::Scene;

pub fn build(aspect_ratio: f64) -> Scene {
    let camera_desc = CameraDescriptor {
        origin: DVec3::new(26.0, 3.0, 6.0),
        look_at: DVec3::new(0.0, 2.0, 0.0),
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
    world.add(XYRect {
        min: DVec2::new(3.0, 1.0),
        max: DVec2::new(5.0, 3.0),
        z: -2.0,
        material: DiffuseLight {
            emit: Solid {
                color: DVec3::new(4.0, 4.0, 4.0),
            },
        },
    });

    Scene {
        world,
        camera,
        background: DVec3::ZERO,
    }
}
