use ::image::{GenericImageView, Pixel, Primitive, Rgb};
use glam::{DVec2, DVec3, UVec2};
use num_traits::ToPrimitive;
use rand::Rng;
use std::{fmt::Debug, sync::Arc};

use crate::perlin::Perlin;

pub trait Texture {
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

#[derive(Debug)]
pub struct Noise {
    perlin: Perlin,
    scale: f64,
}

impl Noise {
    pub fn new<R: Rng>(rng: &mut R, scale: f64) -> Self {
        Self {
            perlin: Perlin::new(rng),
            scale,
        }
    }
}

impl Texture for Noise {
    fn value(&self, _uv: DVec2, point: DVec3) -> DVec3 {
        // DVec3::splat(0.5 * (1.0 + self.perlin.noise(point * self.scale)))
        // DVec3::splat(self.perlin.turbulence(point * self.scale, 7))
        DVec3::splat(
            0.5 * (1.0 + (self.scale * point.z + 10.0 * self.perlin.turbulence(point, 7)).sin()),
        )
    }
}

pub struct Image<I> {
    pub image: I,
}

impl<I: GenericImageView> Texture for Image<I> {
    fn value(&self, uv: DVec2, _point: DVec3) -> DVec3 {
        let dims = UVec2::from(self.image.dimensions());
        let pixel_coordinate = (uv * dims.as_dvec2())
            .clamp(DVec2::ZERO, (dims - 1).as_dvec2())
            .as_uvec2();
        let pixel = self.image.get_pixel(pixel_coordinate.x, pixel_coordinate.y);

        // TODO non-linear color spaces
        let Rgb([r, g, b]) = pixel.to_rgb();
        let r = r.to_f64().unwrap();
        let g = g.to_f64().unwrap();
        let b = b.to_f64().unwrap();
        let min = <I::Pixel as Pixel>::Subpixel::DEFAULT_MIN_VALUE
            .to_f64()
            .unwrap();
        let max = <I::Pixel as Pixel>::Subpixel::DEFAULT_MAX_VALUE
            .to_f64()
            .unwrap();
        (DVec3::new(r, g, b) - min) / (max - min)
    }
}
