use rand::prelude::*;

use crate::aabb::{surrounding_box, AABB};
use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;

pub enum BVH {
    Single {
        left: Box<dyn Hittable>,
        bbox: AABB,
    },
    Double {
        left: Box<dyn Hittable>,
        right: Box<dyn Hittable>,
        bbox: AABB,
    },
}

impl BVH {
    pub fn new(l: &mut Vec<Box<dyn Hittable>>, time0: f32, time1: f32) -> BVH {
        // Note: l is emptied by the this function!
        // l must be non-empty.

        // Pick a random axis and split l in half along it.
        let axis = thread_rng().gen_range(0, 3);
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
                BVH::Single {
                    bbox: left.bounding_box(time0, time1).unwrap(),
                    left,
                }
            }
            2 => {
                let left = l.pop().unwrap();
                let right = l.pop().unwrap();
                BVH::Double {
                    bbox: surrounding_box(
                        left.bounding_box(time0, time1).unwrap(),
                        right.bounding_box(time0, time1).unwrap(),
                    ),
                    left,
                    right,
                }
            }
            _ => {
                let rest = &mut l.split_off(l.len() / 2);
                let left = Box::new(BVH::new(rest, time0, time1));
                let right = Box::new(BVH::new(l, time0, time1));
                BVH::Double {
                    bbox: surrounding_box(
                        left.bounding_box(time0, time1).unwrap(),
                        right.bounding_box(time0, time1).unwrap(),
                    ),
                    left,
                    right,
                }
            }
        }
    }
}

impl Hittable for BVH {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        match self {
            BVH::Single { left, bbox } => {
                if bbox.hit(r, t_min, t_max) {
                    left.hit(r, t_min, t_max)
                } else {
                    None
                }
            }
            BVH::Double { left, right, bbox } => {
                if bbox.hit(r, t_min, t_max) {
                    let left_hit = left.hit(r, t_min, t_max);
                    let right_hit = right.hit(r, t_min, t_max);

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

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        match self {
            BVH::Single { bbox, .. } => Some(bbox.clone()),
            BVH::Double { bbox, .. } => Some(bbox.clone()),
        }
    }
}
