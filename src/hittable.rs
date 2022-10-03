use std::{f64::consts as f64, fmt::Debug, ops::Deref, sync::Arc};

use crate::{material::Material, ray::Ray};
use glam::{DVec2, DVec3, Vec3Swizzles};
use rand::Rng;

#[derive(Clone)]
pub struct HitRecord<'a> {
    pub t: f64,
    pub point: DVec3,
    pub normal: DVec3,
    pub uv: DVec2,
    pub face: Face,
    pub material: &'a dyn Material,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Face {
    Front,
    Back,
}

fn compute_face_normal(ray: &Ray, outward_normal: DVec3) -> (DVec3, Face) {
    if ray.direction.dot(outward_normal) < 0.0 {
        (outward_normal, Face::Front)
    } else {
        (-outward_normal, Face::Back)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Aabb {
    pub minimum: DVec3,
    pub maximum: DVec3,
}

impl Aabb {
    pub fn is_hit_by(&self, ray: &Ray, t_min: f64, t_max: f64) -> bool {
        let a = (self.minimum - ray.origin) / ray.direction;
        let b = (self.maximum - ray.origin) / ray.direction;
        let t0 = a.min(b).max_element().max(t_min);
        let t1 = a.max(b).min_element().min(t_max);
        t0 < t1
    }

    pub fn union(&self, other: &Self) -> Self {
        Self {
            minimum: self.minimum.min(other.minimum),
            maximum: self.maximum.max(other.maximum),
        }
    }

    pub fn offset(&self, offset: DVec3) -> Self {
        Self {
            minimum: self.minimum + offset,
            maximum: self.maximum + offset,
        }
    }
}

pub trait Hittable {
    fn hit<'a>(&'a self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord<'a>>;

    fn bounding_box(&self, start_time: f64, end_time: f64) -> Option<Aabb>;
}

impl<T: Hittable> Hittable for &T {
    fn hit<'a>(&'a self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord<'a>> {
        T::hit(*self, ray, t_min, t_max)
    }

    fn bounding_box(&self, start_time: f64, end_time: f64) -> Option<Aabb> {
        T::bounding_box(*self, start_time, end_time)
    }
}

impl<T: Hittable> Hittable for Box<T> {
    fn hit<'a>(&'a self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord<'a>> {
        T::hit(&self, ray, t_min, t_max)
    }

    fn bounding_box(&self, start_time: f64, end_time: f64) -> Option<Aabb> {
        T::bounding_box(&self, start_time, end_time)
    }
}

//FIXME why is this needed separate from Box<T>?
impl Hittable for Box<dyn Hittable> {
    fn hit<'a>(&'a self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord<'a>> {
        self.deref().hit(ray, t_min, t_max)
    }

    fn bounding_box(&self, start_time: f64, end_time: f64) -> Option<Aabb> {
        self.deref().bounding_box(start_time, end_time)
    }
}

impl<T: Hittable> Hittable for Arc<T> {
    fn hit<'a>(&'a self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord<'a>> {
        T::hit(&self, ray, t_min, t_max)
    }

    fn bounding_box(&self, start_time: f64, end_time: f64) -> Option<Aabb> {
        T::bounding_box(&self, start_time, end_time)
    }
}

impl<T: Hittable> Hittable for [T] {
    fn hit<'a>(&'a self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord<'a>> {
        self.iter()
            .flat_map(|obj| obj.hit(ray, t_min, t_max))
            .filter(|hit| hit.t.is_finite())
            .min_by(|hit1, hit2| {
                hit1.t
                    .partial_cmp(&hit2.t)
                    .expect("incomparable ray parameters")
            })
    }

    fn bounding_box(&self, start_time: f64, end_time: f64) -> Option<Aabb> {
        let (first, rest) = self.split_first()?;
        let mut acc = first.bounding_box(start_time, end_time)?;
        for obj in rest {
            acc = acc.union(&obj.bounding_box(start_time, end_time)?);
        }
        Some(acc)
    }
}

pub struct World {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl World {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add<T: Hittable + 'static>(&mut self, object: T) {
        self.objects.push(Box::new(object));
    }
}

impl Hittable for World {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.objects.hit(ray, t_min, t_max)
    }

    fn bounding_box(&self, start_time: f64, end_time: f64) -> Option<Aabb> {
        self.objects.bounding_box(start_time, end_time)
    }
}

#[derive(Debug)]
pub struct Sphere<Mat> {
    pub center: DVec3,
    pub radius: f64,
    pub material: Mat,
}

impl<Mat: Material> Sphere<Mat> {
    fn get_uv(&self, point: DVec3) -> DVec2 {
        let latitude = point.y.acos();
        let longitude = (-point.z).atan2(point.x) + f64::PI;
        DVec2::new(longitude / f64::TAU, latitude / f64::PI)
    }
}

impl<Mat: Material> Hittable for Sphere<Mat> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let center_to_origin = ray.origin - self.center;

        let a = ray.direction.length_squared();
        let half_b = ray.direction.dot(center_to_origin);
        let c = center_to_origin.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let t1 = (-half_b - discriminant.sqrt()) / a;
        let t2 = (-half_b + discriminant.sqrt()) / a;
        let t = if (t_min..=t_max).contains(&t1) {
            t1
        } else if (t_min..=t_max).contains(&t2) {
            t2
        } else {
            return None;
        };

        let point = ray.at(t);
        let outward_normal = (point - self.center) / self.radius;
        let (normal, face) = compute_face_normal(ray, outward_normal);
        let uv = self.get_uv(outward_normal);

        Some(HitRecord {
            t,
            point,
            normal,
            uv,
            face,
            material: &self.material,
        })
    }

    fn bounding_box(&self, _start_time: f64, _end_time: f64) -> Option<Aabb> {
        Some(Aabb {
            minimum: self.center - DVec3::splat(self.radius),
            maximum: self.center + DVec3::splat(self.radius),
        })
    }
}

/// Encapsulates a hittable in a moving reference frame with the given velocity.
#[derive(Debug)]
pub struct Moving<T> {
    pub velocity: DVec3,
    pub inner: T,
}

impl<T: Hittable> Hittable for Moving<T> {
    fn hit<'a>(&'a self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord<'a>> {
        let inner_ray = Ray {
            origin: ray.origin - self.velocity * ray.time,
            ..*ray
        };
        let hit = self.inner.hit(&inner_ray, t_min, t_max)?;
        Some(HitRecord {
            point: hit.point + self.velocity * ray.time,
            ..hit
        })
    }

    fn bounding_box(&self, start_time: f64, end_time: f64) -> Option<Aabb> {
        let inner_box = self.inner.bounding_box(start_time, end_time)?;
        let start_box = inner_box.offset(self.velocity * start_time);
        let end_box = inner_box.offset(self.velocity * end_time);
        Some(start_box.union(&end_box))
    }
}

pub struct BvhNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    bounding_box: Aabb,
}

impl BvhNode {
    pub fn new(list: &mut [Arc<dyn Hittable>], start_time: f64, end_time: f64) -> Self {
        let axis = rand::thread_rng().gen_range(0..3);
        list.sort_by(|obj_a, obj_b| {
            let box_a = obj_a.bounding_box(start_time, end_time).unwrap();
            let box_b = obj_b.bounding_box(start_time, end_time).unwrap();
            box_a.minimum[axis].total_cmp(&box_b.minimum[axis])
        });
        let [left, right]: [Arc<dyn Hittable>; 2] = match list {
            [a] => [a.clone(), a.clone()],
            [a, b] => [a.clone(), b.clone()],
            _ => {
                let (a_list, b_list) = list.split_at_mut(list.len() / 2);
                [
                    Arc::new(BvhNode::new(a_list, start_time, end_time)),
                    Arc::new(BvhNode::new(b_list, start_time, end_time)),
                ]
            }
        };
        let left_box = left.bounding_box(start_time, end_time).unwrap();
        let right_box = right.bounding_box(start_time, end_time).unwrap();
        let bounding_box = left_box.union(&right_box);
        Self {
            left,
            right,
            bounding_box,
        }
    }
}

impl Hittable for BvhNode {
    fn hit<'a>(&'a self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord<'a>> {
        if !self.bounding_box.is_hit_by(ray, t_min, t_max) {
            return None;
        }

        let left_hit = self.left.hit(ray, t_min, t_max);
        let upper_bound = left_hit
            .as_ref()
            .map(|hit| hit.t.min(t_max))
            .unwrap_or(t_max);
        let right_hit = self.right.hit(ray, t_min, upper_bound);
        // Prioritize right_hit - if right_hit is Some, then it is definitely
        // less than left_hit due to the calculated upper_bound.
        right_hit.or(left_hit)
    }

    fn bounding_box(&self, _start_time: f64, _end_time: f64) -> Option<Aabb> {
        Some(self.bounding_box)
    }
}

pub enum Plane {
    XY,
    YZ,
    ZX,
}

pub struct Rect<Mat> {
    pub plane: Plane,
    pub min: DVec2,
    pub max: DVec2,
    pub k: f64,
    pub material: Mat,
}

impl<Mat: Material> Hittable for Rect<Mat> {
    fn hit<'a>(&'a self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord<'a>> {
        let t = match self.plane {
            Plane::XY => (self.k - ray.origin.z) / ray.direction.z,
            Plane::YZ => (self.k - ray.origin.x) / ray.direction.x,
            Plane::ZX => (self.k - ray.origin.y) / ray.direction.y,
        };
        if t < t_min || t > t_max {
            return None;
        }
        let point = ray.at(t);
        let xy = match self.plane {
            Plane::XY => point.xy(),
            Plane::YZ => point.yz(),
            Plane::ZX => point.zx(),
        };
        if xy.cmplt(self.min).any() || xy.cmpgt(self.max).any() {
            return None;
        }
        let uv = (xy - self.min) / (self.max - self.min);
        let outward_normal = match self.plane {
            Plane::XY => DVec3::Z,
            Plane::YZ => DVec3::X,
            Plane::ZX => DVec3::Y,
        };
        let (normal, face) = compute_face_normal(ray, outward_normal);
        Some(HitRecord {
            t,
            point,
            normal,
            uv,
            face,
            material: &self.material,
        })
    }

    fn bounding_box(&self, _start_time: f64, _end_time: f64) -> Option<Aabb> {
        let epsilon = 0.0001;
        let swizzle = match self.plane {
            Plane::XY => DVec3::xyz,
            Plane::YZ => DVec3::yzx,
            Plane::ZX => DVec3::zxy,
        };
        Some(Aabb {
            minimum: swizzle(self.min.extend(self.k - epsilon)),
            maximum: swizzle(self.max.extend(self.k + epsilon)),
        })
    }
}
