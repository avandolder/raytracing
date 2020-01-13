use rand::prelude::*;

use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Clone, Copy, Debug)]
pub enum Material {
    Diffuse(Vec3),
    Metal(Vec3, f32),
}

fn random_in_unit_sphere() -> Vec3 {
    let mut p = Vec3(1., 1., 1.);
    while p.squared_length() >= 1. {
        let v = Vec3(
            thread_rng().gen::<f32>(),
            thread_rng().gen::<f32>(),
            thread_rng().gen::<f32>(),
        );
        p = 2. * v - Vec3(1., 1., 1.);
    }
    p
}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2. * v.dot(n) * n
}

impl Material {
    pub fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        match *self {
            Material::Diffuse(albedo) => {
                let target = rec.p + rec.normal + random_in_unit_sphere();
                Some((albedo, Ray::new(rec.p, target - rec.p)))
            }
            Material::Metal(albedo, fuzz) => {
                let fuzz = fuzz.min(1.);
                let reflected = reflect(r_in.direction().unit_vector(), rec.normal);
                let scattered = Ray::new(rec.p, reflected + fuzz*random_in_unit_sphere());
                if scattered.direction().dot(rec.normal) > 0. {
                    Some((albedo, scattered))
                } else {
                    None
                }
            }
        }
    }
}
