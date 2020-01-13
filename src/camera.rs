use rand::prelude::*;

use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
    pub lens_radius: f32,
}

fn random_in_unit_disk() -> Vec3 {
    let mut p = Vec3(1., 1., 1.);
    let mut rng = thread_rng();
    while p.squared_length() >= 1. {
        let v = Vec3(rng.gen::<f32>(), rng.gen::<f32>(), 0.);
        p = 2. * v - Vec3(1., 1., 0.);
    }
    p
}

impl Camera {
    pub fn new(
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
        vfov: f32,
        aspect: f32,
        aperture: f32,
        focus_dist: f32,
    ) -> Self {
        let theta = vfov * std::f32::consts::PI / 180.;
        let half_height = (theta / 2.).tan();
        let half_width = aspect * half_height;

        let w = (lookfrom - lookat).unit_vector();
        let u = vup.cross(w).unit_vector();
        let v = w.cross(u);

        Camera {
            origin: lookfrom,
            lower_left_corner: lookfrom
                - half_width * focus_dist * u
                - half_height * focus_dist * v
                - focus_dist * w,
            horizontal: 2. * half_width * focus_dist * u,
            vertical: 2. * half_height * focus_dist * v,
            u,
            v,
            w,
            lens_radius: aperture / 2.,
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x() + self.v * rd.y();
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
        )
    }
}
