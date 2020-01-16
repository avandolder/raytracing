use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct RotateY {
    pub hittable: Box<dyn Hittable>,
    pub sin_theta: f32,
    pub cos_theta: f32,
    pub bbox: Option<AABB>,
}

impl RotateY {
    pub fn new(hittable: impl 'static + Hittable, angle: f32) -> RotateY {
        let radians = (std::f32::consts::PI / 180.) * angle;
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let bbox = hittable.bounding_box(0., 1.).map(|bbox| {
            let mut min = Vec3::new(std::f32::MAX, std::f32::MAX, std::f32::MAX);
            let mut max = Vec3::new(std::f32::MIN, std::f32::MIN, std::f32::MIN);
            for i in 0..2 {
                for j in 0..2 {
                    for k in 0..2 {
                        let (i, j, k) = (i as f32, j as f32, k as f32);
                        let x = i * bbox.max[0] + (1. - i) * bbox.min[0];
                        let y = j * bbox.max[1] + (1. - j) * bbox.min[1];
                        let z = k * bbox.max[2] + (1. - k) * bbox.min[2];
                        let new_x = cos_theta * x + sin_theta * z;
                        let new_z = -sin_theta * x + cos_theta * z;
                        let tester = Vec3::new(new_x, y, new_z);
                        for c in 0..3 {
                            max[c] = max[c].max(tester[c]);
                            min[c] = min[c].min(tester[c]);
                        }
                    }
                }
            }
            AABB::new(min, max)
        });
        RotateY {
            hittable: Box::new(hittable),
            sin_theta,
            cos_theta,
            bbox,
        }
    }
}

impl Hittable for RotateY {
    fn hit<'a>(&'a self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord<'a>> {
        let mut origin = r.origin();
        let mut direction = r.direction();
        origin[0] = self.cos_theta * r.origin()[0] - self.sin_theta * r.origin()[2];
        origin[2] = self.sin_theta * r.origin()[0] + self.cos_theta * r.origin()[2];
        direction[0] = self.cos_theta * r.direction()[0] - self.sin_theta * r.direction()[2];
        direction[2] = self.sin_theta * r.direction()[0] + self.cos_theta * r.direction()[2];

        let rotated_r = Ray::new(origin, direction, r.time());
        self.hittable.hit(&rotated_r, t_min, t_max).map(|rec| {
            let mut p = rec.p;
            let mut normal = rec.normal;
            p[0] = self.cos_theta * rec.p[0] + self.sin_theta * rec.p[2];
            p[2] = -self.sin_theta * rec.p[0] + self.cos_theta * rec.p[2];
            normal[0] = self.cos_theta * rec.normal[0] + self.sin_theta * rec.normal[2];
            normal[2] = -self.sin_theta * rec.normal[0] + self.cos_theta * rec.normal[2];
            HitRecord { p, normal, ..rec }
        })
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        self.bbox.clone()
    }
}
