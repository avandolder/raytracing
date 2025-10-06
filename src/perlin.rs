use std::sync::LazyLock;

use rand::{Rng, seq::SliceRandom};

use crate::vec3::Vec3;

static RANVEC: LazyLock<Vec<Vec3>> = LazyLock::new(|| perlin_generate(256));
static PERM_X: LazyLock<Vec<i32>> = LazyLock::new(|| perlin_permutation(256));
static PERM_Y: LazyLock<Vec<i32>> = LazyLock::new(|| perlin_permutation(256));
static PERM_Z: LazyLock<Vec<i32>> = LazyLock::new(|| perlin_permutation(256));

fn perlin_permutation(n: usize) -> Vec<i32> {
    let mut p = (0..n as i32).collect::<Vec<i32>>();
    p.shuffle(&mut rand::rng());
    p
}

fn perlin_generate(n: usize) -> Vec<Vec3> {
    let mut rng = rand::rng();
    (0..n)
        .map(move |_| {
            let x = 2. * rng.random::<f32>() - 1.;
            let y = 2. * rng.random::<f32>() - 1.;
            let z = 2. * rng.random::<f32>() - 1.;
            Vec3::new(x, y, z).unit_vector()
        })
        .collect()
}

#[inline(always)]
fn perlin_interp(c: [[[Vec3; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {
    let uu = u * u * (3. - 2. * u);
    let vv = v * v * (3. - 2. * v);
    let ww = w * w * (3. - 2. * w);
    itertools::iproduct!(0..2, 0..2, 0..2)
        .map(|(i, j, k)| {
            let (ii, jj, kk) = (i as f32, j as f32, k as f32);
            let weight_v = Vec3::new(u - ii, v - jj, w - kk);
            (ii * uu + (1. - ii) * (1. - uu))
                * (jj * vv + (1. - jj) * (1. - vv))
                * (kk * ww + (1. - kk) * (1. - ww))
                * c[i][j][k].dot(weight_v)
        })
        .sum()
}

pub fn turbulence(mut p: Vec3, depth: usize) -> f32 {
    let mut weight = 1.;
    (0..depth)
        .map(|_| {
            let turb = noise(p) * weight;
            weight *= 0.5;
            p *= 2.;
            turb
        })
        .sum::<f32>()
        .abs()
}

pub fn noise(p: Vec3) -> f32 {
    let u = p.x() - p.x().floor();
    let v = p.y() - p.y().floor();
    let w = p.z() - p.z().floor();

    let i = p.x().floor() as i32;
    let j = p.y().floor() as i32;
    let k = p.z().floor() as i32;

    let c: [[[Vec3; 2]; 2]; 2] = std::array::from_fn(|di| {
        std::array::from_fn(|dj| {
            std::array::from_fn(move |dk| {
                RANVEC[(PERM_X[((i + di as i32) & 255) as usize]
                    ^ PERM_Y[((j + dj as i32) & 255) as usize]
                    ^ PERM_Z[((k + dk as i32) & 255) as usize]) as usize]
            })
        })
    });
    perlin_interp(c, u, v, w)
}
