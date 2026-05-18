use rand::prelude::*;
use rand_chacha::ChaCha12Rng;

use crate::aabb::{Aabb, surrounding_box};
use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;

pub enum Bvh {
    Single {
        left: Box<Hittable>,
        bbox: Aabb,
    },
    Double {
        left: Box<Hittable>,
        right: Box<Hittable>,
        bbox: Aabb,
    },
}

impl Bvh {
    pub fn new(rng: &mut impl Rng, l: &mut Vec<Hittable>, time0: f32, time1: f32) -> Bvh {
        // Note: l is emptied by the this function!
        // l must be non-empty.

        // Pick a random axis and split l in half along it.
        let axis = rng.random_range(0..3);
        l.sort_unstable_by(|a, b| {
            let bbox_left = a.bounding_box(0., 0.).expect("No AABB in BVH constructor!");
            let bbox_right = b.bounding_box(0., 0.).expect("No AABB in BVH constructor!");

            bbox_left.min[axis]
                .partial_cmp(&bbox_right.min[axis])
                .unwrap()
                .reverse()
        });

        // The current BVH node will either directly contain one/two hittables, or
        // it will contain two BVH children nodes.
        match l.len() {
            0 => panic!("BVH cannot be created with 0 nodes!"),
            1 => {
                let left = l.pop().unwrap();
                Bvh::Single {
                    bbox: left.bounding_box(time0, time1).unwrap(),
                    left: left.into(),
                }
            }
            2 => {
                let left = l.pop().unwrap();
                let right = l.pop().unwrap();
                Bvh::Double {
                    bbox: surrounding_box(
                        left.bounding_box(time0, time1).unwrap(),
                        right.bounding_box(time0, time1).unwrap(),
                    ),
                    left: left.into(),
                    right: right.into(),
                }
            }
            _ => {
                let rest = &mut l.split_off(l.len() / 2);
                let left = Bvh::new(rng, rest, time0, time1);
                let right = Bvh::new(rng, l, time0, time1);
                Bvh::Double {
                    bbox: surrounding_box(
                        left.bounding_box(time0, time1).unwrap(),
                        right.bounding_box(time0, time1).unwrap(),
                    ),
                    left: Box::new(left.into()),
                    right: Box::new(right.into()),
                }
            }
        }
    }

    pub fn hit(
        &self,
        rng: &mut ChaCha12Rng,
        r: &Ray,
        t_min: f32,
        t_max: f32,
    ) -> Option<HitRecord<'_>> {
        match self {
            Bvh::Single { left, bbox } => bbox
                .hit(r, t_min, t_max)
                .then(|| left.hit(rng, r, t_min, t_max))?,
            Bvh::Double { left, right, bbox } => {
                if bbox.hit(r, t_min, t_max) {
                    let left_hit = left.hit(rng, r, t_min, t_max);
                    let right_hit = right.hit(rng, r, t_min, t_max);

                    match (left_hit, right_hit) {
                        (Some(left_rec), Some(right_rec)) => {
                            if left_rec.t < right_rec.t {
                                Some(left_rec)
                            } else {
                                Some(right_rec)
                            }
                        }
                        (Some(left_rec), _) => Some(left_rec),
                        (_, Some(right_rec)) => Some(right_rec),
                        _ => None,
                    }
                } else {
                    None
                }
            }
        }
    }

    pub fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<Aabb> {
        match self {
            Bvh::Single { bbox, .. } | Bvh::Double { bbox, .. } => Some(bbox.clone()),
        }
    }
}
