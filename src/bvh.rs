use rand::prelude::*;

use crate::aabb::{surrounding_box, AABB};
use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;

pub struct BVH {
    pub left: Option<Box<dyn Hittable>>,
    pub right: Option<Box<dyn Hittable>>,
    pub bbox: AABB,
}

impl BVH {
    pub fn new(l: &mut Vec<Box<dyn Hittable>>, time0: f32, time1: f32) -> BVH {
        let axis = thread_rng().gen_range(0, 3);
        l.sort_unstable_by(|a, b| {
            let bbox_left = a.bounding_box(0., 0.).expect("No AABB in BVH constructor!");
            let bbox_right = b.bounding_box(0., 0.).expect("No AABB in BVH constructor!");

            bbox_left.min[axis]
                .partial_cmp(&bbox_right.min[axis])
                .unwrap()
                .reverse()
        });

        let n = l.len();
        let (left, right) = match n {
            1 => (Some(l.pop().unwrap()), None),
            2 => (Some(l.pop().unwrap()), Some(l.pop().unwrap())),
            _ => {
                let mut rest = l.split_off(n / 2);
                (
                    Some(Box::new(BVH::new(&mut rest, time0, time1)) as Box<dyn Hittable>),
                    Some(Box::new(BVH::new(l, time0, time1)) as Box<dyn Hittable>),
                )
            }
        };

        let bbox = if left.is_some() && right.is_some() {
            let bbox_left = left.as_ref().unwrap().bounding_box(time0, time1).unwrap();
            let bbox_right = right.as_ref().unwrap().bounding_box(time0, time1).unwrap();
            surrounding_box(bbox_left, bbox_right)
        } else {
            left.as_ref().unwrap().bounding_box(time0, time1).unwrap()
        };

        BVH { left, right, bbox }
    }
}

impl Hittable for BVH {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        if self.bbox.hit(r, t_min, t_max) {
            let left = self.left.as_ref().and_then(|x| x.hit(r, t_min, t_max));
            let right = self.right.as_ref().and_then(|x| x.hit(r, t_min, t_max));

            match (left, right) {
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

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        Some(self.bbox.clone())
    }
}
