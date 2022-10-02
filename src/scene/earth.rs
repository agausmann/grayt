use glam::DVec3;

use crate::{
    camera::{Camera, CameraDescriptor},
    hittable::{Sphere, World},
    material::Lambertian,
    texture::Image,
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
    world.add(Sphere {
        center: DVec3::ZERO,
        radius: 2.0,
        material: Lambertian {
            albedo: Image {
                image: image::open("assets/earthmap.jpg").unwrap(),
            },
        },
    });

    Scene { world, camera }
}
