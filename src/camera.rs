use glam::DVec3;

use crate::ray::Ray;

#[derive(Debug, Clone)]
pub struct CameraDescriptor {
    pub vfov: f64,
    pub aspect_ratio: f64,
    pub origin: DVec3,
}

impl Default for CameraDescriptor {
    fn default() -> Self {
        Self {
            vfov: 90.0,
            aspect_ratio: 16.0 / 9.0,
            origin: DVec3::ZERO,
        }
    }
}

pub struct Camera {
    origin: DVec3,
    lower_left: DVec3,
    horizontal: DVec3,
    vertical: DVec3,
}

impl Camera {
    pub fn new(desc: &CameraDescriptor) -> Self {
        let origin = desc.origin;
        let theta = desc.vfov.to_radians();
        let h = (theta / 2.0).tan();

        let viewport_height = 2.0 * h;
        let viewport_width = desc.aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let horizontal = viewport_width * DVec3::X;
        let vertical = viewport_height * DVec3::Y;
        let outward = focal_length * DVec3::Z;
        let lower_left = -(horizontal + vertical) / 2.0 - outward;

        Self {
            origin,
            horizontal,
            vertical,
            lower_left,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.lower_left + u * self.horizontal + v * self.vertical,
        }
    }
}
