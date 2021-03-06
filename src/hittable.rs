use crate::aabb::{surrounding_box, AABB};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct HitRecord<'a> {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub mat: &'a Material,
    pub u: f32,
    pub v: f32,
}

pub trait Hittable {
    fn hit<'a>(&'a self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord<'a>>;
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB>;
}

impl Hittable for Vec<Box<dyn Hittable>> {
    fn hit<'a>(&'a self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord<'a>> {
        self.iter()
            .filter_map(|item| item.hit(r, t_min, t_max))
            .min_by(|r1, r2| r1.t.partial_cmp(&r2.t).unwrap())
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        if let Some(init) = self.get(0).and_then(|x| x.bounding_box(t0, t1)) {
            self.iter().try_fold(init, |box1, item| {
                item.bounding_box(t0, t1)
                    .map(|box2| surrounding_box(box1, box2))
            })
        } else {
            None
        }
    }
}

pub struct FlipNormals(Box<dyn Hittable>);

impl Hittable for FlipNormals {
    fn hit<'a>(&'a self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord<'a>> {
        if let Some(rec) = self.0.hit(r, t_min, t_max) {
            Some(HitRecord {
                normal: -rec.normal,
                ..rec
            })
        } else {
            None
        }
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        self.0.bounding_box(t0, t1)
    }
}

pub fn flip_normals<T: 'static + Hittable>(hittable: T) -> Box<FlipNormals> {
    Box::new(FlipNormals(Box::new(hittable)))
}
