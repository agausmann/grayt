use glam::{DVec3, IVec3};
use rand::{distributions::Uniform, Rng};

const POINT_COUNT: usize = 256;

#[derive(Debug)]
pub struct Perlin {
    ranfloat: Vec<f64>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
}

impl Perlin {
    pub fn new<R: Rng>(rng: &mut R) -> Self {
        let float_distr = Uniform::new_inclusive(0.0, 1.0);
        let ranfloat: Vec<f64> = rng.sample_iter(float_distr).take(POINT_COUNT).collect();
        let mut perm_x: Vec<usize> = (0..POINT_COUNT).collect();
        let mut perm_y = perm_x.clone();
        let mut perm_z = perm_x.clone();
        shuffle(&mut perm_x, rng);
        shuffle(&mut perm_y, rng);
        shuffle(&mut perm_z, rng);

        Self {
            ranfloat,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    pub fn noise(&self, point: DVec3) -> f64 {
        let uvw = point.fract();
        let uvw = uvw * uvw * (3.0 - 2.0 * uvw);
        let ijk = point.floor().as_ivec3();

        let mut c = [[[0.0; 2]; 2]; 2];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] =
                        self.int_noise(ijk + IVec3::new(di as i32, dj as i32, dk as i32));
                }
            }
        }
        trilinear(c, uvw)
    }

    pub fn int_noise(&self, idx: IVec3) -> f64 {
        self.ranfloat[self.perm_x[idx.x as usize & 0xff]
            ^ self.perm_y[idx.y as usize & 0xff]
            ^ self.perm_z[idx.z as usize & 0xff]]
    }
}

fn shuffle<R: Rng, T>(items: &mut [T], rng: &mut R) {
    for i in (1..POINT_COUNT).rev() {
        let target = rng.gen_range(0..=i);
        items.swap(i, target);
    }
}

fn trilinear(source: [[[f64; 2]; 2]; 2], uvw: DVec3) -> f64 {
    let DVec3 { x: u, y: v, z: w } = uvw;

    let mut accum = 0.0;
    for (i, ki) in (0..2).zip([1.0 - u, u]) {
        for (j, kj) in (0..2).zip([1.0 - v, v]) {
            for (k, kk) in (0..2).zip([1.0 - w, w]) {
                accum += ki * kj * kk * source[i][j][k];
            }
        }
    }
    accum
}
