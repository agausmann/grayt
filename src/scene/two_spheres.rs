use std::sync::Arc;

use glam::DVec3;

use crate::{
    camera::{Camera, CameraDescriptor},
    hittable::{Sphere, World},
    material::Lambertian,
    texture::{Checker, Solid},
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

    let mut world = World::new();

    let checker = Arc::new(Lambertian {
        albedo: Checker {
            even: Solid {
                color: DVec3::new(0.2, 0.3, 0.1),
            },
            odd: Solid {
                color: DVec3::new(0.9, 0.9, 0.9),
            },
        },
    });

    world.add(Sphere {
        center: -10.0 * DVec3::Y,
        radius: 10.0,
        material: Arc::clone(&checker),
    });
    world.add(Sphere {
        center: 10.0 * DVec3::Y,
        radius: 10.0,
        material: Arc::clone(&checker),
    });

    Scene { world, camera }
}
