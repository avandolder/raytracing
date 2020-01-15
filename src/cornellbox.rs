use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable, flip_normals};
use crate::material::Material;
use crate::ray::Ray;
use crate::rectangle::{XYRect, XZRect, YZRect};
use crate::vec3::Vec3;

pub struct CornellBox {
    pub pmin: Vec3,
    pub pmax: Vec3,
    pub sides: Vec<Box<dyn Hittable>>,
}

impl CornellBox {
    pub fn new(p0: impl Into<Vec3>, p1: impl Into<Vec3>, mat: Material) -> CornellBox {
        let (p0, p1) = (p0.into(), p1.into());
        CornellBox {
            pmin: p0,
            pmax: p1,
            sides: vec![
                Box::new(XYRect::new(p0.x(), p1.x(), p0.y(), p1.y(), p1.z(), mat.clone())),
                flip_normals(XYRect::new(p0.x(), p1.x(), p0.y(), p1.y(), p0.z(), mat.clone())),
                Box::new(XZRect::new(p0.x(), p1.x(), p0.z(), p1.z(), p1.y(), mat.clone())),
                flip_normals(XZRect::new(p0.x(), p1.x(), p0.z(), p1.z(), p0.y(), mat.clone())),
                Box::new(YZRect::new(p0.y(), p1.y(), p0.z(), p1.z(), p1.x(), mat.clone())),
                flip_normals(YZRect::new(p0.y(), p1.y(), p0.z(), p1.z(), p0.x(), mat.clone())),
            ],
        }
    }
}

impl Hittable for CornellBox {
    fn hit<'a>(&self, r: &Ray, t0: f32, t1: f32) -> Option<HitRecord> {
        self.sides.hit(r, t0, t1)
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        Some(AABB::new(self.pmin, self.pmax))
    }
}
