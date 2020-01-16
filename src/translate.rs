use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Translate {
    hittable: Box<dyn Hittable>,
    offset: Vec3,
}

impl Translate {
    pub fn new(hittable: impl 'static + Hittable, offset: impl Into<Vec3>) -> Translate {
        Translate {
            hittable: Box::new(hittable),
            offset: offset.into(),
        }
    }
}

impl Hittable for Translate {
    fn hit<'a>(&'a self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord<'a>> {
        let moved_r = Ray::new(r.origin() - self.offset, r.direction(), r.time());
        self.hittable
            .hit(&moved_r, t_min, t_max)
            .map(|rec| HitRecord {
                p: rec.p + self.offset,
                ..rec
            })
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        self.hittable
            .bounding_box(t0, t1)
            .map(|bbox| AABB::new(bbox.min + self.offset, bbox.max + self.offset))
    }
}
