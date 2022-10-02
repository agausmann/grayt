use glam::{DVec2, DVec3};
use std::{fmt::Debug, sync::Arc};

pub trait Texture: Debug {
    fn value(&self, uv: DVec2, point: DVec3) -> DVec3;
}

impl<T: Texture> Texture for &T {
    fn value(&self, uv: DVec2, point: DVec3) -> DVec3 {
        T::value(*self, uv, point)
    }
}

impl<T: Texture> Texture for Arc<T> {
    fn value(&self, uv: DVec2, point: DVec3) -> DVec3 {
        T::value(&*self, uv, point)
    }
}

#[derive(Debug, Clone)]
pub struct Solid {
    pub color: DVec3,
}

impl Texture for Solid {
    fn value(&self, _uv: DVec2, _point: DVec3) -> DVec3 {
        self.color
    }
}

#[derive(Debug, Clone)]
pub struct Checker<Even, Odd> {
    pub even: Even,
    pub odd: Odd,
}

impl<Even, Odd> Texture for Checker<Even, Odd>
where
    Even: Texture,
    Odd: Texture,
{
    fn value(&self, uv: DVec2, point: DVec3) -> DVec3 {
        let sines = (10.0 * point.x).sin() * (10.0 * point.y).sin() * (10.0 * point.z).sin();
        if sines < 0.0 {
            self.odd.value(uv, point)
        } else {
            self.even.value(uv, point)
        }
    }
}
