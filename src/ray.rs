use glam::DVec3;

#[derive(Debug, Clone)]
pub struct Ray {
    pub origin: DVec3,
    pub direction: DVec3,
    pub time: f64,
}

impl Ray {
    pub fn at(&self, t: f64) -> DVec3 {
        self.origin + self.direction * t
    }
}
