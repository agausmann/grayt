use crate::ray::Ray;
use glam::Vec3;

pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f32,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
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
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
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
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
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
        let normal = (point - self.center) / self.radius;
        Some(HitRecord { point, normal, t })
    }
}
