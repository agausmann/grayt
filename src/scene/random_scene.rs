use std::sync::Arc;

use glam::DVec3;
use rand::Rng;

use crate::{
    camera::{Camera, CameraDescriptor},
    hittable::{BvhNode, Hittable, Moving, Sphere, World},
    material::{Dielectric, Lambertian, Metal},
    texture::{Checker, Solid},
};

use super::Scene;

pub fn build(aspect_ratio: f64) -> Scene {
    let camera_desc = CameraDescriptor {
        origin: DVec3::new(13.0, 2.0, 3.0),
        look_at: DVec3::ZERO,
        vfov: 20.0,
        aspect_ratio,
        aperture: 0.1,
        focus_distance: Some(10.0),
        shutter_time: 1.0,
        ..Default::default()
    };
    let camera = Camera::new(&camera_desc);

    let mut objects: Vec<Arc<dyn Hittable>> = Vec::new();

    objects.push(Arc::new(Sphere {
        center: DVec3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Lambertian {
            albedo: Checker {
                odd: Solid {
                    color: DVec3::new(0.2, 0.3, 0.1),
                },
                even: Solid {
                    color: DVec3::new(0.9, 0.9, 0.9),
                },
            },
        },
    }));

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
                    albedo: Solid {
                        color: DVec3::new(
                            rng.gen_range(0.0..1.0) * rng.gen_range(0.0..1.0),
                            rng.gen_range(0.0..1.0) * rng.gen_range(0.0..1.0),
                            rng.gen_range(0.0..1.0) * rng.gen_range(0.0..1.0),
                        ),
                    },
                };
                let velocity = DVec3::new(0.0, rng.gen_range(0.0..0.5), 0.0);
                objects.push(Arc::new(Moving {
                    velocity,
                    inner: Sphere {
                        center,
                        radius,
                        material,
                    },
                }));
            } else if choose_mat < 0.95 {
                let material = Metal {
                    albedo: DVec3::new(
                        rng.gen_range(0.5..1.0),
                        rng.gen_range(0.5..1.0),
                        rng.gen_range(0.5..1.0),
                    ),
                    fuzz: rng.gen_range(0.0..0.5),
                };
                objects.push(Arc::new(Sphere {
                    center,
                    radius,
                    material,
                }));
            } else {
                let material = Dielectric { ir: 1.5 };
                objects.push(Arc::new(Sphere {
                    center,
                    radius,
                    material,
                }));
            }
        }
    }

    objects.push(Arc::new(Sphere {
        center: DVec3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        material: Dielectric { ir: 1.5 },
    }));
    objects.push(Arc::new(Sphere {
        center: DVec3::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: Lambertian {
            albedo: Solid {
                color: DVec3::new(0.4, 0.2, 0.1),
            },
        },
    }));
    objects.push(Arc::new(Sphere {
        center: DVec3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material: Metal {
            albedo: DVec3::new(0.7, 0.6, 0.5),
            fuzz: 0.0,
        },
    }));

    let mut world = World::new();
    world.add(BvhNode::new(&mut objects, 0.0, camera_desc.shutter_time));

    Scene {
        world,
        camera,
        background: DVec3::new(0.7, 0.8, 1.0),
    }
}
