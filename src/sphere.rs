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
    fn hit<'a>(&'a self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord<'a>> {
        let oc = r.origin() - self.center;
        let a = r.direction().squared_length();
        let b = oc.dot(r.direction());
        let c = oc.dot(oc) - self.radius * self.radius;

        let discriminant = b * b - a * c;
        if discriminant > 0. {
            let t = (-b - discriminant.sqrt()) / a;
            if t < t_max && t > t_min {
                let p = r.point_at_parameter(t);
                let (u, v) = get_sphere_uv((p - self.center) / self.radius);
                return Some(HitRecord {
                    t,
                    p,
                    normal: (p - self.center) / self.radius,
                    mat: &self.mat,
                    u,
                    v,
                });
            }

            let t = (-b + discriminant.sqrt()) / a;
            if t < t_max && t > t_min {
                let p = r.point_at_parameter(t);
                let (u, v) = get_sphere_uv((p - self.center) / self.radius);
                return Some(HitRecord {
                    t,
                    p,
                    normal: (p - self.center) / self.radius,
                    mat: &self.mat,
                    u,
                    v,
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

pub fn get_sphere_uv(p: Vec3) -> (f32, f32) {
    use std::f32::consts::PI;
    let phi = p.z().atan2(p.x());
    let theta = p.y().asin();
    let u = 1. - (phi + PI) / (2. * PI);
    let v = (theta + PI / 2.) / PI;
    (u, v)
}
