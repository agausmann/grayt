use glam::Vec3;

use crate::ray::Ray;

#[derive(Debug, Clone)]
pub struct CameraDescriptor {
    pub viewport_width: f32,
    pub viewport_height: f32,
    pub focal_length: f32,
    pub origin: Vec3,
}

impl Default for CameraDescriptor {
    fn default() -> Self {
        let aspect_ratio = 16.0 / 9.0;
        Self {
            viewport_height: 2.0,
            viewport_width: 2.0 * aspect_ratio,
            focal_length: 1.0,
            origin: Vec3::ZERO,
        }
    }
}

pub struct Camera {
    origin: Vec3,
    lower_left: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(desc: &CameraDescriptor) -> Self {
        let origin = desc.origin;
        let horizontal = desc.viewport_width * Vec3::X;
        let vertical = desc.viewport_height * Vec3::Y;
        let outward = desc.focal_length * Vec3::Z;
        let lower_left = -(horizontal + vertical) / 2.0 - outward;

        Self {
            origin,
            horizontal,
            vertical,
            lower_left,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.lower_left + u * self.horizontal + v * self.vertical,
        }
    }
}
