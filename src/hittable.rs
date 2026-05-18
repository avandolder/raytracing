use rand_chacha::ChaCha12Rng;

use crate::aabb::Aabb;
use crate::bvh::Bvh;
use crate::constant_medium::ConstantMedium;
use crate::cornellbox::CornellBox;
use crate::material::Material;
use crate::moving_sphere::MovingSphere;
use crate::ray::Ray;
use crate::rectangle::{XYRect, XZRect, YZRect};
use crate::rotate::RotateY;
use crate::sphere::Sphere;
use crate::vec3::Vec3;

pub struct HitRecord<'a> {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub mat: &'a Material,
    pub u: f32,
    pub v: f32,
}

pub enum Hittable {
    Translate(Box<Hittable>, Vec3),
    FlipNormals(Box<Hittable>),

    Bvh(Bvh),
    Sphere(Sphere),
    MovingSphere(MovingSphere),
    CornellBox(CornellBox),
    ConstantMedium(ConstantMedium),
    RotateY(RotateY),
    XYRect(XYRect),
    XZRect(XZRect),
    YZRect(YZRect),
}

impl Hittable {
    pub fn hit<'a>(
        &'a self,
        rng: &mut ChaCha12Rng,
        r: &Ray,
        t_min: f32,
        t_max: f32,
    ) -> Option<HitRecord<'a>> {
        use Hittable::*;
        match self {
            Translate(hittable, offset) => {
                let moved_r = Ray::new(r.origin() - *offset, r.direction(), r.time());
                hittable
                    .hit(rng, &moved_r, t_min, t_max)
                    .map(|rec| HitRecord {
                        p: rec.p + *offset,
                        ..rec
                    })
            }
            FlipNormals(hittable) => hittable.hit(rng, r, t_min, t_max).map(|rec| HitRecord {
                normal: -rec.normal,
                ..rec
            }),

            Bvh(bvh) => bvh.hit(rng, r, t_min, t_max),
            Sphere(sphere) => sphere.hit(r, t_min, t_max),
            MovingSphere(moving_sphere) => moving_sphere.hit(r, t_min, t_max),
            CornellBox(cornell_box) => cornell_box.hit(rng, r, t_min, t_max),
            ConstantMedium(constant_medium) => constant_medium.hit(rng, r, t_min, t_max),
            RotateY(rotate_y) => rotate_y.hit(rng, r, t_min, t_max),
            XYRect(xyrect) => xyrect.hit(r, t_min, t_max),
            XZRect(xzrect) => xzrect.hit(r, t_min, t_max),
            YZRect(yzrect) => yzrect.hit(r, t_min, t_max),
        }
    }

    pub fn bounding_box(&self, t0: f32, t1: f32) -> Option<Aabb> {
        use Hittable::*;
        match self {
            Translate(hittable, offset) => hittable
                .bounding_box(t0, t1)
                .map(|bbox| Aabb::new(bbox.min + *offset, bbox.max + *offset)),
            FlipNormals(hittable) => hittable.bounding_box(t0, t1),

            Bvh(bvh) => bvh.bounding_box(t0, t1),
            Sphere(sphere) => sphere.bounding_box(t0, t1),
            MovingSphere(moving_sphere) => moving_sphere.bounding_box(t0, t1),
            CornellBox(cornell_box) => cornell_box.bounding_box(t0, t1),
            ConstantMedium(constant_medium) => constant_medium.bounding_box(t0, t1),
            RotateY(rotate_y) => rotate_y.bounding_box(t0, t1),
            XYRect(xyrect) => xyrect.bounding_box(t0, t1),
            XZRect(xzrect) => xzrect.bounding_box(t0, t1),
            YZRect(yzrect) => yzrect.bounding_box(t0, t1),
        }
    }

    pub fn translate(hittable: impl Into<Hittable>, offset: impl Into<Vec3>) -> Hittable {
        Hittable::Translate(Box::new(hittable.into()), offset.into())
    }

    pub fn flip_normals(hittable: impl Into<Hittable>) -> Hittable {
        Hittable::FlipNormals(Box::new(hittable.into()))
    }
}

pub fn hit_group<'a>(
    hittables: &'a [Hittable],
    rng: &mut ChaCha12Rng,
    r: &Ray,
    t_min: f32,
    t_max: f32,
) -> Option<HitRecord<'a>> {
    hittables
        .iter()
        .filter_map(|item| item.hit(rng, r, t_min, t_max).filter(|r| !r.t.is_nan()))
        .min_by(|r1, r2| r1.t.partial_cmp(&r2.t).unwrap())
}

impl From<Bvh> for Hittable {
    fn from(value: Bvh) -> Self {
        Hittable::Bvh(value)
    }
}

macro_rules! gen_from {
    ($t:ident $($ts:ident)*) => {
        gen_from!($($ts)*);
        impl From<$t> for Hittable {
            fn from(value: $t) -> Self {
                Hittable::$t(value)
            }
        }
    };
    () => {}
}
gen_from!(Sphere MovingSphere CornellBox ConstantMedium RotateY XYRect XZRect YZRect);
