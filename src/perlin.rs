use lazy_static::lazy_static;
use rand::{seq::SliceRandom, Rng};

use crate::vec3::Vec3;

lazy_static! {
    static ref RANVEC: Vec<Vec3> = perlin_generate(256);
    static ref PERM_X: Vec<i32> = perlin_permutation(256);
    static ref PERM_Y: Vec<i32> = perlin_permutation(256);
    static ref PERM_Z: Vec<i32> = perlin_permutation(256);
}

fn perlin_permutation(n: usize) -> Vec<i32> {
    let mut p = (0..n as i32).collect::<Vec<i32>>();
    p.shuffle(&mut rand::thread_rng());
    p
}

fn perlin_generate(n: usize) -> Vec<Vec3> {
    let mut p = Vec::with_capacity(n);
    let mut rng = rand::thread_rng();

    for _ in 0..n {
        let x = 2. * rng.gen::<f32>() - 1.;
        let y = 2. * rng.gen::<f32>() - 1.;
        let z = 2. * rng.gen::<f32>() - 1.;
        p.push(Vec3::new(x, y, z).unit_vector())
    }

    p
}

#[inline(always)]
fn perlin_interp(c: [[[Vec3; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {
    let uu = u * u * (3. - 2. * u);
    let vv = v * v * (3. - 2. * v);
    let ww = w * w * (3. - 2. * w);
    let mut accum = 0.;
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let (ii, jj, kk) = (i as f32, j as f32, k as f32);
                let weight_v = Vec3::new(u - ii, v - jj, w - kk);
                accum += (ii * uu + (1. - ii) * (1. - uu))
                    * (jj * vv + (1. - jj) * (1. - vv))
                    * (kk * ww + (1. - kk) * (1. - ww))
                    * c[i][j][k].dot(weight_v);
            }
        }
    }
    accum
}

pub fn turbulence(mut p: Vec3, depth: usize) -> f32 {
    let mut accum = 0.;
    let mut weight = 1.;
    for _ in 0..depth {
        accum += noise(p) * weight;
        weight *= 0.5;
        p *= 2.;
    }
    accum.abs()
}

pub fn noise(p: Vec3) -> f32 {
    let u = p.x() - p.x().floor();
    let v = p.y() - p.y().floor();
    let w = p.z() - p.z().floor();

    let i = p.x().floor() as i32;
    let j = p.y().floor() as i32;
    let k = p.z().floor() as i32;

    let mut c: [[[Vec3; 2]; 2]; 2] = [[[Vec3::default(); 2]; 2]; 2];
    for di in 0..2 {
        for dj in 0..2 {
            for dk in 0..2 {
                c[di][dj][dk] = RANVEC[(PERM_X[((i + di as i32) & 255) as usize]
                    ^ PERM_Y[((j + dj as i32) & 255) as usize]
                    ^ PERM_Z[((k + dk as i32) & 255) as usize])
                    as usize];
            }
        }
    }
    perlin_interp(c, u, v, w)
}
