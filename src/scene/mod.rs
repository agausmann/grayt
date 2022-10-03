pub mod cornell_box;
pub mod earth;
pub mod random_scene;
pub mod simple_light;
pub mod two_perlin_spheres;
pub mod two_spheres;

use glam::DVec3;

use crate::{camera::Camera, hittable::World};

pub struct Scene {
    pub world: World,
    pub camera: Camera,
    pub background: DVec3,
}
