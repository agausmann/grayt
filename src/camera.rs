use glam::{DVec2, DVec3};
use rand::{
    distributions::{Distribution, Uniform},
    Rng,
};

use crate::ray::Ray;

fn random_in_unit_disk<R: Rng>(rng: &mut R) -> DVec2 {
    let dist = Uniform::new_inclusive(-1.0, 1.0);
    loop {
        let candidate = DVec2::new(dist.sample(rng), dist.sample(rng));
        if candidate.length_squared() <= 1.0 {
            return candidate;
        }
    }
}

#[derive(Debug, Clone)]
pub struct CameraDescriptor {
    pub vfov: f64,
    pub aspect_ratio: f64,
    pub origin: DVec3,
    pub look_at: DVec3,
    pub vup: DVec3,
    pub aperture: f64,
    pub focus_distance: Option<f64>,
}

impl Default for CameraDescriptor {
    fn default() -> Self {
        Self {
            vfov: 90.0,
            aspect_ratio: 16.0 / 9.0,
            origin: DVec3::ZERO,
            look_at: DVec3::NEG_Z,
            vup: DVec3::Y,
            aperture: 0.0,
            focus_distance: None,
        }
    }
}

pub struct Camera {
    origin: DVec3,
    lower_left: DVec3,
    horizontal: DVec3,
    vertical: DVec3,
    u: DVec3,
    v: DVec3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(desc: &CameraDescriptor) -> Self {
        let origin = desc.origin;
        let theta = desc.vfov.to_radians();
        let h = (theta / 2.0).tan();

        let focus_distance = desc
            .focus_distance
            .unwrap_or_else(|| desc.origin.distance(desc.look_at));

        let viewport_height = 2.0 * h;
        let viewport_width = desc.aspect_ratio * viewport_height;

        let w = (desc.origin - desc.look_at).normalize();
        let u = desc.vup.cross(w).normalize();
        let v = w.cross(u);

        let horizontal = focus_distance * viewport_width * u;
        let vertical = focus_distance * viewport_height * v;
        let outward = focus_distance * w;
        let lower_left = -(horizontal + vertical) / 2.0 - outward;
        let lens_radius = desc.aperture / 2.0;

        Self {
            origin,
            horizontal,
            vertical,
            lower_left,
            u,
            v,
            lens_radius,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let mut rng = rand::thread_rng();
        let rd = self.lens_radius * random_in_unit_disk(&mut rng);
        let offset = self.u * rd.x + self.v * rd.y;

        Ray {
            origin: self.origin + offset,
            direction: self.lower_left + u * self.horizontal + v * self.vertical - offset,
        }
    }
}
