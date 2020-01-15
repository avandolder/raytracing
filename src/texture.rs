use crate::perlin;
use crate::vec3::Vec3;

#[derive(Clone, Debug)]
pub enum Texture {
    Checker {
        odd: Box<Texture>,
        even: Box<Texture>,
    },
    Image {
        data: Vec<u8>,
        w: u32,
        h: u32,
    },
    Noise {
        scale: f32,
    },
    Solid {
        color: Vec3,
    },
}

impl Texture {
    pub fn checker(odd: Texture, even: Texture) -> Texture {
        Texture::Checker {
            odd: Box::new(odd),
            even: Box::new(even),
        }
    }

    pub fn noise(scale: f32) -> Texture {
        Texture::Noise { scale }
    }

    pub fn solid<V: Into<Vec3>>(color: V) -> Texture {
        Texture::Solid {
            color: color.into(),
        }
    }

    pub fn value(&self, u: f32, v: f32, p: Vec3) -> Vec3 {
        match self {
            Texture::Checker { odd, even } => {
                let sines = (10. * p.x()).sin() * (10. * p.y()).sin() * (10. * p.z()).sin();
                if sines < 0. {
                    odd.value(u, v, p)
                } else {
                    even.value(u, v, p)
                }
            }
            Texture::Image { data, w, h } => {
                let (w, h) = (*w as usize, *h as usize);
                let i = ((u * w as f32) as i32).max(0).min(w as i32 - 1) as usize;
                let j = (((1. - v) * h as f32 - 0.001) as i32).max(0).min(h as i32 - 1) as usize;
                let (r, g, b) = (
                    data[3*i + 3*w*j] as f32 / 255.,
                    data[3*i + 3*w*j + 1] as f32 / 255.,
                    data[3*i + 3*w*j + 2] as f32 / 255.,
                );
                Vec3::new(r, g, b)
            }
            Texture::Noise { scale } => {
                Vec3::new(1., 1., 1.)
                    * (1. + (scale * p.z() + 10. * perlin::turbulence(p, 7)).sin())
                    * 0.5
            }
            Texture::Solid { color } => *color,
        }
    }
}
