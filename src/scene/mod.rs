pub mod random_scene;

use crate::{camera::Camera, hittable::World};

pub struct Scene {
    pub world: World,
    pub camera: Camera,
}
