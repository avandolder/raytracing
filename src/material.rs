use rand::prelude::*;

use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::texture::Texture;
use crate::vec3::Vec3;

#[derive(Clone, Debug)]
pub enum Material {
    Glass(f32),
    Diffuse(Texture),
    Light(Texture),
    Metal(Vec3, f32),
    Isotropic(Texture),
}

fn random_in_unit_sphere(rng: &mut impl Rng) -> Vec3 {
    let mut p = Vec3::new(1., 1., 1.);
    while p.squared_length() >= 1. {
        let v = Vec3::new(
            rng.random::<f32>(),
            rng.random::<f32>(),
            rng.random::<f32>(),
        );
        p = 2. * v - Vec3::new(1., 1., 1.);
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
    (discriminant > 0.).then(|| ni_over_nt * (uv - n * dt) - n * discriminant.sqrt())
}

fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let r0 = ((1. - ref_idx) / (1. + ref_idx)).powf(2.);
    r0 + (1. - r0) * (1. - cosine).powf(5.)
}

impl Material {
    pub fn scatter(&self, rng: &mut impl Rng, r_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        match self {
            Material::Glass(ref_idx) => {
                let reflected = reflect(r_in.direction(), rec.normal);
                let (outward_normal, ni_over_nt, cosine) = if r_in.direction().dot(rec.normal) > 0.
                {
                    let cosine =
                        ref_idx * r_in.direction().dot(rec.normal) / r_in.direction().length();
                    (-rec.normal, *ref_idx, cosine)
                } else {
                    let cosine = -r_in.direction().dot(rec.normal) / r_in.direction().length();
                    (rec.normal, 1. / *ref_idx, cosine)
                };

                Some((
                    Vec3::new(1., 1., 1.),
                    refract(r_in.direction(), outward_normal, ni_over_nt).map_or(
                        Ray::new(rec.p, reflected, r_in.time()),
                        |refracted| {
                            Ray::new(
                                rec.p,
                                (rng.random::<f32>() < schlick(cosine, *ref_idx))
                                    .then_some(reflected)
                                    .unwrap_or(refracted),
                                r_in.time(),
                            )
                        },
                    ),
                ))
            }
            Material::Diffuse(albedo) => {
                let target = rec.p + rec.normal + random_in_unit_sphere(rng);
                Some((
                    albedo.value(rec.u, rec.v, rec.p),
                    Ray::new(rec.p, target - rec.p, r_in.time()),
                ))
            }
            Material::Light(_) => None,
            Material::Metal(albedo, fuzz) => {
                let fuzz = fuzz.min(1.);
                let reflected = reflect(r_in.direction().unit_vector(), rec.normal);
                let scattered = Ray::new(
                    rec.p,
                    reflected + fuzz * random_in_unit_sphere(rng),
                    r_in.time(),
                );
                (scattered.direction().dot(rec.normal) > 0.).then_some((*albedo, scattered))
            }
            Material::Isotropic(texture) => Some((
                texture.value(rec.u, rec.v, rec.p),
                Ray::new(rec.p, random_in_unit_sphere(rng), r_in.time()),
            )),
        }
    }

    pub fn emitted(&self, u: f32, v: f32, p: Vec3) -> Vec3 {
        match self {
            Material::Light(emit) => emit.value(u, v, p),
            _ => Vec3::new(0., 0., 0.),
        }
    }
}
