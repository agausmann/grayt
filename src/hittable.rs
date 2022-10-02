use std::{fmt::Debug, sync::Arc};

use crate::{material::Material, ray::Ray};
use glam::DVec3;
use rand::Rng;

#[derive(Debug, Clone)]
pub struct HitRecord<'a> {
    pub t: f64,
    pub point: DVec3,
    pub normal: DVec3,
    pub is_front_face: bool,
    pub material: &'a dyn Material,
}

impl<'a> HitRecord<'a> {
    pub fn from_outward_normal(
        t: f64,
        point: DVec3,
        ray: &Ray,
        outward_normal: DVec3,
        material: &'a dyn Material,
    ) -> Self {
        let is_front_face = ray.direction.dot(outward_normal) < 0.0;
        let normal = if is_front_face {
            outward_normal
        } else {
            -outward_normal
        };
        Self {
            t,
            point,
            normal,
            is_front_face,
            material,
        }
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

pub trait Hittable: Debug {
    fn hit<'a>(&'a self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord<'a>>;

    fn bounding_box(&self, start_time: f64, end_time: f64) -> Option<Aabb>;
}

#[derive(Debug)]
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
        self.objects
            .iter()
            .flat_map(|obj| obj.hit(ray, t_min, t_max))
            .filter(|hit| hit.t.is_finite())
            .min_by(|hit1, hit2| {
                hit1.t
                    .partial_cmp(&hit2.t)
                    .expect("incomparable ray parameters")
            })
    }

    fn bounding_box(&self, start_time: f64, end_time: f64) -> Option<Aabb> {
        let (first, rest) = self.objects.split_first()?;
        let mut acc = first.bounding_box(start_time, end_time)?;
        for obj in rest {
            acc = acc.union(&obj.bounding_box(start_time, end_time)?);
        }
        Some(acc)
    }
}

#[derive(Debug)]
pub struct Sphere<Mat> {
    pub center: DVec3,
    pub radius: f64,
    pub material: Mat,
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

        Some(HitRecord::from_outward_normal(
            t,
            point,
            ray,
            outward_normal,
            &self.material,
        ))
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

#[derive(Debug)]
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
