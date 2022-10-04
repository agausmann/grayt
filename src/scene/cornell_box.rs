use std::sync::Arc;

use glam::{DVec2, DVec3};

use crate::{
    camera::{Camera, CameraDescriptor},
    hittable::{Cuboid, Plane, Rect, RotateY, Translate, World},
    material::{DiffuseLight, Lambertian},
    texture::Solid,
};

use super::Scene;

pub fn build(aspect_ratio: f64) -> Scene {
    let camera_desc = CameraDescriptor {
        aspect_ratio,
        origin: DVec3::new(278.0, 278.0, -800.0),
        look_at: DVec3::new(278.0, 278.0, 0.0),
        vfov: 40.0,
        ..Default::default()
    };
    let camera = Camera::new(&camera_desc);

    let red = Arc::new(Lambertian {
        albedo: Solid {
            color: DVec3::new(0.65, 0.05, 0.05),
        },
    });
    let white = Arc::new(Lambertian {
        albedo: Solid {
            color: DVec3::new(0.73, 0.73, 0.73),
        },
    });
    let green = Arc::new(Lambertian {
        albedo: Solid {
            color: DVec3::new(0.12, 0.45, 0.15),
        },
    });
    let light = Arc::new(DiffuseLight {
        emit: Solid {
            color: DVec3::new(15.0, 15.0, 15.0),
        },
    });

    let mut world = World::new();
    world.add(Rect {
        plane: Plane::YZ,
        min: DVec2::new(0.0, 0.0),
        max: DVec2::new(555.0, 555.0),
        k: 555.0,
        material: Arc::clone(&green),
    });
    world.add(Rect {
        plane: Plane::YZ,
        min: DVec2::new(0.0, 0.0),
        max: DVec2::new(555.0, 555.0),
        k: 0.0,
        material: Arc::clone(&red),
    });
    world.add(Rect {
        plane: Plane::ZX,
        min: DVec2::new(227.0, 213.0),
        max: DVec2::new(332.0, 343.0),
        k: 554.0,
        material: Arc::clone(&light),
    });
    world.add(Rect {
        plane: Plane::ZX,
        min: DVec2::new(0.0, 0.0),
        max: DVec2::new(555.0, 555.0),
        k: 0.0,
        material: Arc::clone(&white),
    });
    world.add(Rect {
        plane: Plane::ZX,
        min: DVec2::new(0.0, 0.0),
        max: DVec2::new(555.0, 555.0),
        k: 555.0,
        material: Arc::clone(&white),
    });
    world.add(Rect {
        plane: Plane::XY,
        min: DVec2::new(0.0, 0.0),
        max: DVec2::new(555.0, 555.0),
        k: 555.0,
        material: Arc::clone(&white),
    });

    world.add(Translate {
        offset: DVec3::new(265.0, 0.0, 295.0),
        inner: RotateY {
            radians: 15.0_f64.to_radians(),
            inner: Cuboid::new(
                DVec3::ZERO,
                DVec3::new(165.0, 330.0, 165.0),
                Arc::clone(&white),
            ),
        },
    });
    world.add(Translate {
        offset: DVec3::new(130.0, 0.0, 65.0),
        inner: RotateY {
            radians: (-18.0_f64).to_radians(),
            inner: Cuboid::new(
                DVec3::ZERO,
                DVec3::new(165.0, 165.0, 165.0),
                Arc::clone(&white),
            ),
        },
    });

    Scene {
        world,
        camera,
        background: DVec3::ZERO,
    }
}
