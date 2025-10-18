use rand::{Rng, rng};

use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable},
    material::Material,
    ray::Ray,
    texture::Texture,
    vec3::Vec3,
};

pub struct ConstantMedium {
    boundary: Box<Hittable>,
    neg_inv_density: f32,
    phase_function: Material,
}

impl ConstantMedium {
    pub fn new(boundary: impl Into<Hittable>, density: f32, tex: Texture) -> Self {
        Self {
            boundary: Box::new(boundary.into()),
            neg_inv_density: -1. / density,
            phase_function: Material::Isotropic(tex),
        }
    }

    pub fn hit<'a>(&'a self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord<'a>> {
        let mut rec1 = self.boundary.hit(r, f32::NEG_INFINITY, f32::INFINITY)?;
        let mut rec2 = self.boundary.hit(r, rec1.t + 0.0001, f32::INFINITY)?;

        rec1.t = rec1.t.max(t_min);
        rec2.t = rec2.t.min(t_max);
        if rec1.t >= rec2.t {
            return None;
        }

        rec1.t = rec1.t.max(0.);

        let ray_length = r.direction().length();
        let dist_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * rng().random::<f32>().ln();
        if hit_distance > dist_inside_boundary {
            return None;
        }

        let t = rec1.t + hit_distance / ray_length;
        Some(HitRecord {
            t,
            p: r.point_at_parameter(t),
            normal: Vec3::new(1., 0., 0.),
            mat: &self.phase_function,
            u: 0.,
            v: 0.,
        })
    }

    pub fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        self.boundary.bounding_box(t0, t1)
    }
}
