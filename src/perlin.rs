use std::iter::repeat_with;

use glam::{DVec3, IVec3};
use rand::{distributions::Uniform, prelude::Distribution, Rng};

const POINT_COUNT: usize = 256;

#[derive(Debug)]
pub struct Perlin {
    ranvec: Vec<DVec3>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
}

impl Perlin {
    pub fn new<R: Rng>(rng: &mut R) -> Self {
        let float_distr = Uniform::new_inclusive(-1.0, 1.0);
        let random_vec = || {
            DVec3::new(
                float_distr.sample(rng),
                float_distr.sample(rng),
                float_distr.sample(rng),
            )
        };
        let ranvec: Vec<DVec3> = repeat_with(random_vec).take(POINT_COUNT).collect();
        let mut perm_x: Vec<usize> = (0..POINT_COUNT).collect();
        let mut perm_y = perm_x.clone();
        let mut perm_z = perm_x.clone();
        shuffle(&mut perm_x, rng);
        shuffle(&mut perm_y, rng);
        shuffle(&mut perm_z, rng);

        Self {
            ranvec,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    pub fn noise(&self, point: DVec3) -> f64 {
        let uvw = point.fract();
        let uvw_smooth = uvw * uvw * (3.0 - 2.0 * uvw);
        let (uu, vv, ww) = uvw_smooth.into();

        let ijk = point.floor().as_ivec3();

        let mut accum = 0.0;
        for (di, ki) in (0..2).zip([1.0 - uu, uu]) {
            for (dj, kj) in (0..2).zip([1.0 - vv, vv]) {
                for (dk, kk) in (0..2).zip([1.0 - ww, ww]) {
                    let dijk = IVec3::new(di, dj, dk);
                    let weight = uvw - dijk.as_dvec3();
                    let sample = self.int_noise(ijk + dijk);
                    accum += ki * kj * kk * sample.dot(weight);
                }
            }
        }
        accum
    }

    fn int_noise(&self, idx: IVec3) -> DVec3 {
        let idx = idx.as_uvec3() % POINT_COUNT as u32;
        self.ranvec[self.perm_x[idx.x as usize]
            ^ self.perm_y[idx.y as usize]
            ^ self.perm_z[idx.z as usize]]
    }
}

fn shuffle<R: Rng, T>(items: &mut [T], rng: &mut R) {
    for i in (1..POINT_COUNT).rev() {
        let target = rng.gen_range(0..=i);
        items.swap(i, target);
    }
}
