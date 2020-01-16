use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct XYRect {
    pub mat: Material,
    x0: f32,
    x1: f32,
    y0: f32,
    y1: f32,
    k: f32,
}

impl XYRect {
    pub fn new(x0: f32, x1: f32, y0: f32, y1: f32, k: f32, mat: Material) -> XYRect {
        XYRect {
            x0,
            x1,
            y0,
            y1,
            k,
            mat,
        }
    }
}

impl Hittable for XYRect {
    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        Some(AABB::new(
            Vec3::new(self.x0, self.y0, self.k - 0.0001),
            Vec3::new(self.x1, self.y1, self.k + 0.0001),
        ))
    }

    fn hit<'a>(&'a self, r: &Ray, t0: f32, t1: f32) -> Option<HitRecord> {
        let t = (self.k - r.origin().z()) / r.direction().z();
        if t < t0 || t > t1 {
            return None;
        }

        let x = r.origin().x() + t * r.direction().x();
        let y = r.origin().y() + t * r.direction().y();
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }

        Some(HitRecord {
            t,
            u: (x - self.x0) / (self.x1 - self.x0),
            v: (y - self.y0) / (self.y1 - self.y0),
            mat: &self.mat,
            p: r.point_at_parameter(t),
            normal: Vec3::new(0., 0., 1.),
        })
    }
}

pub struct XZRect {
    pub mat: Material,
    x0: f32,
    x1: f32,
    z0: f32,
    z1: f32,
    k: f32,
}

impl XZRect {
    pub fn new(x0: f32, x1: f32, z0: f32, z1: f32, k: f32, mat: Material) -> XZRect {
        XZRect {
            x0,
            x1,
            z0,
            z1,
            k,
            mat,
        }
    }
}

impl Hittable for XZRect {
    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        Some(AABB::new(
            Vec3::new(self.x0, self.k - 0.0001, self.z0),
            Vec3::new(self.x1, self.k + 0.0001, self.z1),
        ))
    }

    fn hit<'a>(&'a self, r: &Ray, t0: f32, t1: f32) -> Option<HitRecord> {
        let t = (self.k - r.origin().y()) / r.direction().y();
        if t < t0 || t > t1 {
            return None;
        }

        let x = r.origin().x() + t * r.direction().x();
        let z = r.origin().z() + t * r.direction().z();
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return None;
        }

        Some(HitRecord {
            t,
            u: (x - self.x0) / (self.x1 - self.x0),
            v: (z - self.z0) / (self.z1 - self.z0),
            mat: &self.mat,
            p: r.point_at_parameter(t),
            normal: Vec3::new(0., 1., 0.),
        })
    }
}

pub struct YZRect {
    pub mat: Material,
    y0: f32,
    y1: f32,
    z0: f32,
    z1: f32,
    k: f32,
}

impl YZRect {
    pub fn new(y0: f32, y1: f32, z0: f32, z1: f32, k: f32, mat: Material) -> YZRect {
        YZRect {
            y0,
            y1,
            z0,
            z1,
            k,
            mat,
        }
    }
}

impl Hittable for YZRect {
    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        Some(AABB::new(
            Vec3::new(self.k - 0.0001, self.y0, self.z0),
            Vec3::new(self.k + 0.0001, self.y1, self.z1),
        ))
    }

    fn hit<'a>(&'a self, r: &Ray, t0: f32, t1: f32) -> Option<HitRecord> {
        let t = (self.k - r.origin().x()) / r.direction().x();
        if t < t0 || t > t1 {
            return None;
        }

        let y = r.origin().y() + t * r.direction().y();
        let z = r.origin().z() + t * r.direction().z();
        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return None;
        }

        Some(HitRecord {
            t,
            u: (y - self.y0) / (self.y1 - self.y0),
            v: (z - self.z0) / (self.z1 - self.z0),
            mat: &self.mat,
            p: r.point_at_parameter(t),
            normal: Vec3::new(1., 0., 0.),
        })
    }
}
