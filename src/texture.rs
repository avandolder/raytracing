use crate::vec3::Vec3;

#[derive(Clone, Debug)]
pub enum Texture {
    Solid { color: Vec3 },
    Checker {
        odd: Box<Texture>,
        even: Box<Texture>,
    },
}

impl Texture {
    pub fn solid<V: Into<Vec3>>(color: V) -> Texture {
        Texture::Solid { color: color.into() }
    }

    pub fn checker(odd: Texture, even: Texture) -> Texture {
        Texture::Checker {
            odd: Box::new(odd),
            even: Box::new(even),
        }
    }

    pub fn value(&self, u: f32, v: f32, p: Vec3) -> Vec3 {
        match self {
            Texture::Solid { color } => *color,
            Texture::Checker { odd, even } => {
                let sines = (10.*p.x()).sin() * (10.*p.y()).sin() * (10.*p.z()).sin();
                if sines < 0. {
                    odd.value(u, v, p)
                } else {
                    even.value(u, v, p)
                }
            }
        }
    }
}
