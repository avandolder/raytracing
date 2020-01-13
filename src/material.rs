use rand::prelude::*;

use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Clone, Copy, Debug)]
pub enum Material {
    Dielectric(f32),
    Diffuse(Vec3),
    Metal(Vec3, f32),
}

fn random_in_unit_sphere() -> Vec3 {
    let mut p = Vec3(1., 1., 1.);
    let mut rng = thread_rng();
    while p.squared_length() >= 1. {
        let v = Vec3(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>());
        p = 2. * v - Vec3(1., 1., 1.);
    }
    p
}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2. * v.dot(n) * n
}

fn refract(v: Vec3, n: Vec3, ni_over_nt: f32) -> Option<Vec3> {
    let uv = v.unit_vector();
    let dt = uv.dot(n);
    let discriminant = 1. - ni_over_nt * ni_over_nt * (1. - dt * dt);
    if discriminant > 0. {
        Some(ni_over_nt * (uv - n * dt) - n * discriminant.sqrt())
    } else {
        None
    }
}

fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let r0 = ((1. - ref_idx) / (1. + ref_idx)).powf(2.);
    r0 + (1. - r0) * (1. - cosine).powf(5.)
}

impl Material {
    pub fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        match *self {
            Material::Dielectric(ref_idx) => {
                let reflected = reflect(r_in.direction(), rec.normal);
                let (outward_normal, ni_over_nt, cosine) = if r_in.direction().dot(rec.normal) > 0.
                {
                    let cosine =
                        ref_idx * r_in.direction().dot(rec.normal) / r_in.direction().length();
                    (-rec.normal, ref_idx, cosine)
                } else {
                    let cosine = -r_in.direction().dot(rec.normal) / r_in.direction().length();
                    (rec.normal, 1. / ref_idx, cosine)
                };

                if let Some(refracted) = refract(r_in.direction(), outward_normal, ni_over_nt) {
                    let reflect_prob = schlick(cosine, ref_idx);
                    if thread_rng().gen::<f32>() < reflect_prob {
                        Some((Vec3(1., 1., 1.), Ray::new(rec.p, reflected)))
                    } else {
                        Some((Vec3(1., 1., 1.), Ray::new(rec.p, refracted)))
                    }
                } else {
                    Some((Vec3(1., 1., 1.), Ray::new(rec.p, reflected)))
                }
            }
            Material::Diffuse(albedo) => {
                let target = rec.p + rec.normal + random_in_unit_sphere();
                Some((albedo, Ray::new(rec.p, target - rec.p)))
            }
            Material::Metal(albedo, fuzz) => {
                let fuzz = fuzz.min(1.);
                let reflected = reflect(r_in.direction().unit_vector(), rec.normal);
                let scattered = Ray::new(rec.p, reflected + fuzz * random_in_unit_sphere());
                if scattered.direction().dot(rec.normal) > 0. {
                    Some((albedo, scattered))
                } else {
                    None
                }
            }
        }
    }
}
