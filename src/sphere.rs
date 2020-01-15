use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Clone)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub mat: Material,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, mat: Material) -> Sphere {
        Sphere {
            center,
            radius,
            mat,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
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
                    normal: (p - self.center) / self.radius,
                    mat: self.mat.clone(),
                });
            }

            let t = (-b + discriminant.sqrt()) / a;
            if t < t_max && t > t_min {
                let p = r.point_at_parameter(t);
                return Some(HitRecord {
                    t,
                    p,
                    normal: (p - self.center) / self.radius,
                    mat: self.mat.clone(),
                });
            }
        }

        None
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        Some(AABB::new(
            self.center - Vec3::new(self.radius, self.radius, self.radius),
            self.center + Vec3::new(self.radius, self.radius, self.radius),
        ))
    }
}
