use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}

impl Camera {
    pub fn new(vfov: f32, aspect: f32) -> Self {
        let theta = vfov * std::f32::consts::PI / 180.;
        let half_height = (theta / 2.).tan();
        let half_width = aspect * half_height;

        Camera {
            lower_left_corner: Vec3(-half_width, -half_height, -1.),
            horizontal: Vec3(2. * half_width, 0., 0.),
            vertical: Vec3(0., 2. * half_height, 0.),
            origin: Vec3(0., 0., 0.),
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
}
