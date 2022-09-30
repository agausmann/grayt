use glam::DVec3;
use rand::{
    distributions::{Distribution, Uniform},
    Rng,
};

use crate::{hittable::HitRecord, ray::Ray};

#[allow(dead_code)]
fn random_in_unit_sphere<R: Rng>(rng: &mut R) -> DVec3 {
    let dist = Uniform::new_inclusive(-1.0, 1.0);
    loop {
        let candidate = DVec3::new(dist.sample(rng), dist.sample(rng), dist.sample(rng));
        if candidate.length_squared() <= 1.0 {
            return candidate;
        }
    }
}

#[allow(dead_code)]
fn random_on_unit_sphere<R: Rng>(rng: &mut R) -> DVec3 {
    loop {
        let candidate = random_in_unit_sphere(rng);
        if let Some(unit) = candidate.try_normalize() {
            return unit;
        }
    }
}

#[allow(dead_code)]
fn random_on_hemisphere<R: Rng>(rng: &mut R, normal: DVec3) -> DVec3 {
    let unit = random_on_unit_sphere(rng);
    if unit.dot(normal) > 0.0 {
        unit
    } else {
        -unit
    }
}

fn is_near_zero(v: DVec3) -> bool {
    let eps = 1.0e-8;
    v.abs().cmplt(DVec3::splat(eps)).all()
}

pub struct Scatter {
    pub ray: Ray,
    pub attenuation: DVec3,
}

pub trait Material {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<Scatter>;
}

impl<M: Material> Material for &M {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<Scatter> {
        M::scatter(*self, ray, hit)
    }
}

#[derive(Debug, Clone)]
pub struct Lambertian {
    pub albedo: DVec3,
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit: &HitRecord) -> Option<Scatter> {
        let mut direction = hit.normal + random_on_unit_sphere(&mut rand::thread_rng());
        if is_near_zero(direction) {
            direction = hit.normal;
        }

        Some(Scatter {
            ray: Ray {
                origin: hit.point,
                direction,
            },
            attenuation: self.albedo,
        })
    }
}
