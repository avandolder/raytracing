use crate::vec3::Vec3;

#[derive(Debug, Default)]
pub struct Ray {
    pub a: Vec3,
    pub b: Vec3,
    _time: f32,
}

#[rustfmt::skip]
impl Ray {
    pub fn new(a: Vec3, b: Vec3, time: f32) -> Ray {
        Ray { a, b, _time: time }
    }

    pub fn origin(&self) -> Vec3 { self.a }
    pub fn direction(&self) -> Vec3 { self.b }
    pub fn time(&self) -> f32 { self._time }
    pub fn point_at_parameter(&self, t: f32) -> Vec3 {
        self.a + self.b * t
    }
}
