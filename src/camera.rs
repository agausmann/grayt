use glam::DVec3;

use crate::ray::Ray;

#[derive(Debug, Clone)]
pub struct CameraDescriptor {
    pub viewport_width: f64,
    pub viewport_height: f64,
    pub focal_length: f64,
    pub origin: DVec3,
}

impl Default for CameraDescriptor {
    fn default() -> Self {
        let aspect_ratio = 16.0 / 9.0;
        Self {
            viewport_height: 2.0,
            viewport_width: 2.0 * aspect_ratio,
            focal_length: 1.0,
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
        let horizontal = desc.viewport_width * DVec3::X;
        let vertical = desc.viewport_height * DVec3::Y;
        let outward = desc.focal_length * DVec3::Z;
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
