use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct MovingSphere {
    pub center0: Vec3,
    pub center1: Vec3,
    pub time0: f32,
    pub time1: f32,
    pub radius: f32,
    pub mat: Material,
}

impl MovingSphere {
    pub fn new(
        center0: Vec3,
        center1: Vec3,
        time0: f32,
        time1: f32,
        radius: f32,
        mat: Material,
    ) -> Self {
        MovingSphere {
            center0,
            center1,
            time0,
            time1,
            radius,
            mat,
        }
    }

    pub fn center(&self, time: f32) -> Vec3 {
        self.center0
            + ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = r.origin() - self.center(r.time());
        let a = r.direction().squared_length();
        let b = oc.dot(r.direction());
        let c = oc.dot(oc) - self.radius * self.radius;

        let discriminant = b * b - a * c;
        if discriminant > 0. {
            let t = (-b - discriminant.sqrt()) / a;
            if t < t_max && t > t_min {
                let p = r.point_at_parameter(t);
                return Some(HitRecord {
                    t,
                    p,
                    normal: (p - self.center(r.time())) / self.radius,
                    mat: self.mat,
                });
            }

            let t = (-b + discriminant.sqrt()) / a;
            if t < t_max && t > t_min {
                let p = r.point_at_parameter(t);
                return Some(HitRecord {
                    t,
                    p,
                    normal: (p - self.center(r.time())) / self.radius,
                    mat: self.mat,
                });
            }
        }

        None
    }
}