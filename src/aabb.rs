use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Clone, Debug)]
pub struct AABB {
    pub min: Vec3,
    pub max: Vec3,
}

impl AABB {
    pub fn new(min: Vec3, max: Vec3) -> AABB {
        AABB { min, max }
    }

    pub fn hit(&self, r: &Ray, mut tmin: f32, mut tmax: f32) -> bool {
        std::iter::zip(r.direction().0, r.origin().0)
            .zip(self.min.0)
            .zip(self.max.0)
            .all(|(((d, o), min), max)| {
                let invd = 1. / d;
                let (m0, m1) = if invd >= 0. { (min, max) } else { (max, min) };
                let t0 = (m0 - o) * invd;
                let t1 = (m1 - o) * invd;

                tmin = t0.max(tmin);
                tmax = t1.min(tmax);
                tmax > tmin
            })
    }
}

pub fn surrounding_box(box0: AABB, box1: AABB) -> AABB {
    let small = Vec3::new(
        box0.min.x().min(box1.min.x()),
        box0.min.y().min(box1.min.y()),
        box0.min.z().min(box1.min.z()),
    );
    let big = Vec3::new(
        box0.max.x().max(box1.max.x()),
        box0.max.y().max(box1.max.y()),
        box0.max.z().max(box1.max.z()),
    );
    AABB::new(small, big)
}
