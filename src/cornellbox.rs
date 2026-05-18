use rand_chacha::ChaCha12Rng;

use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable, hit_group};
use crate::material::Material;
use crate::ray::Ray;
use crate::rectangle::{XYRect, XZRect, YZRect};
use crate::vec3::Vec3;

pub struct CornellBox {
    pub pmin: Vec3,
    pub pmax: Vec3,
    pub sides: Vec<Hittable>,
}

impl CornellBox {
    pub fn new(p0: impl Into<Vec3>, p1: impl Into<Vec3>, mat: Material) -> CornellBox {
        let (p0, p1) = (p0.into(), p1.into());
        CornellBox {
            pmin: p0,
            pmax: p1,
            sides: vec![
                Hittable::XYRect(XYRect::new(
                    p0.x(),
                    p1.x(),
                    p0.y(),
                    p1.y(),
                    p1.z(),
                    mat.clone(),
                )),
                Hittable::flip_normals(XYRect::new(
                    p0.x(),
                    p1.x(),
                    p0.y(),
                    p1.y(),
                    p0.z(),
                    mat.clone(),
                )),
                Hittable::XZRect(XZRect::new(
                    p0.x(),
                    p1.x(),
                    p0.z(),
                    p1.z(),
                    p1.y(),
                    mat.clone(),
                )),
                Hittable::flip_normals(XZRect::new(
                    p0.x(),
                    p1.x(),
                    p0.z(),
                    p1.z(),
                    p0.y(),
                    mat.clone(),
                )),
                Hittable::YZRect(YZRect::new(
                    p0.y(),
                    p1.y(),
                    p0.z(),
                    p1.z(),
                    p1.x(),
                    mat.clone(),
                )),
                Hittable::flip_normals(YZRect::new(
                    p0.y(),
                    p1.y(),
                    p0.z(),
                    p1.z(),
                    p0.x(),
                    mat.clone(),
                )),
            ],
        }
    }

    pub fn hit(&self, rng: &mut ChaCha12Rng, r: &Ray, t0: f32, t1: f32) -> Option<HitRecord<'_>> {
        hit_group(&self.sides, rng, r, t0, t1)
    }

    pub fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<Aabb> {
        Some(Aabb::new(self.pmin, self.pmax))
    }
}
