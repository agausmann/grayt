pub mod earth;
pub mod random_scene;
pub mod two_perlin_spheres;
pub mod two_spheres;

use crate::{camera::Camera, hittable::World};

pub struct Scene {
    pub world: World,
    pub camera: Camera,
}
