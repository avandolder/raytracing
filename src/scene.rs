use image::GenericImageView as _;
use itertools::iproduct;
use rand::Rng;

use crate::{
    Config,
    bvh::BVH,
    camera::Camera,
    constant_medium::ConstantMedium,
    cornellbox::CornellBox,
    hittable::{Hittable, flip_normals},
    material::Material,
    moving_sphere::MovingSphere,
    rectangle::{XYRect, XZRect, YZRect},
    rotate::RotateY,
    sphere::Sphere,
    texture::Texture,
    translate::Translate,
    vec3::Vec3,
};

pub struct Scene {
    pub(crate) geometry: BVH,
    pub(crate) camera: Camera,
    pub(crate) use_ambient_light: bool,
}

pub fn random_scene(config: &Config, rng: &mut impl Rng) -> Scene {
    let n = 500;
    let mut world: Vec<Box<dyn Hittable + Sync>> = Vec::with_capacity(n + 1);

    let checker = Texture::checker(
        Texture::solid((0.2, 0.3, 0.1)),
        Texture::solid((0.9, 0.9, 0.9)),
    );
    world.push(Box::new(Sphere::new(
        Vec3::new(0., -1000., 0.),
        1000.,
        Material::Diffuse(checker),
    )));

    world.extend(iproduct!(-11..11, -11..11).map(|(a, b)| {
        let center = loop {
            let center = Vec3::new(
                a as f32 + 0.9 * rng.random::<f32>(),
                0.2,
                b as f32 + 0.9 * rng.random::<f32>(),
            );
            if (center - Vec3::new(4., 0.2, 0.)).length() > 0.9 {
                break center;
            }
        };

        let choose_mat = rng.random::<f32>();
        if choose_mat < 0.25 {
            Box::new(MovingSphere::new(
                center,
                center + Vec3::new(0., 0.5 * rng.random::<f32>(), 0.),
                0.,
                1.,
                0.2,
                Material::Diffuse(Texture::solid(Vec3::new(
                    rng.random::<f32>() * rng.random::<f32>(),
                    rng.random::<f32>() * rng.random::<f32>(),
                    rng.random::<f32>() * rng.random::<f32>(),
                ))),
            )) as Box<dyn Hittable + Sync>
        } else if choose_mat < 0.5 {
            Box::new(ConstantMedium::new(
                Sphere::new(center, 0.2, Material::Diffuse(Texture::solid((1., 1., 1.)))),
                0.01,
                Texture::solid(Vec3::new(
                    rng.random::<f32>() * rng.random::<f32>(),
                    rng.random::<f32>() * rng.random::<f32>(),
                    rng.random::<f32>() * rng.random::<f32>(),
                )),
            )) as Box<dyn Hittable + Sync>
        } else if choose_mat < 0.65 {
            Box::new(Sphere::new(
                center,
                0.2,
                Material::Diffuse(Texture::noise(4.)),
            ))
        } else if choose_mat < 0.95 {
            Box::new(Sphere::new(
                center,
                0.2,
                Material::Metal(
                    Vec3::new(
                        0.5 * (1. + rng.random::<f32>()),
                        0.5 * (1. + rng.random::<f32>()),
                        0.5 * (1. + rng.random::<f32>()),
                    ),
                    0.5 * rng.random::<f32>(),
                ),
            ))
        } else {
            Box::new(Sphere::new(center, 0.2, Material::Glass(1.5)))
        }
    }));

    world.push(Box::new(Sphere::new(
        Vec3::new(0., 1., 0.),
        1.,
        Material::Glass(1.5),
    )));

    let img = image::open("earthmap.jpg").unwrap();
    let data = img.to_rgb8().into_raw();
    let (w, h) = img.dimensions();
    world.push(Box::new(Sphere::new(
        Vec3::new(4., 1., 0.),
        1.,
        Material::Diffuse(Texture::Image { data, w, h }),
    )));

    world.push(Box::new(Sphere::new(
        Vec3::new(-4., 1., 0.),
        1.,
        Material::Metal(Vec3::new(0.7, 0.6, 0.5), 0.),
    )));

    Scene {
        geometry: BVH::new(rng, &mut world, 0., 1.),
        camera: Camera::new(
            Vec3::new(13., 2., 3.),
            Vec3::new(0., 0., 0.),
            Vec3::new(0., 1., 0.),
            20.,
            config.aspect_ratio,
            0.,
            10.,
            0.,
            1.,
        ),
        use_ambient_light: true,
    }
}

pub fn cornell_box(config: &Config, rng: &mut impl Rng) -> Scene {
    let red = Material::Diffuse(Texture::solid((0.65, 0.05, 0.05)));
    let white = Material::Diffuse(Texture::solid((0.73, 0.73, 0.73)));
    let green = Material::Diffuse(Texture::solid((0.12, 0.45, 0.15)));
    let light = Material::Light(Texture::solid((15., 15., 15.)));

    let lookfrom = Vec3::new(278., 278., -800.);
    let lookat = Vec3::new(278., 278., 0.);
    let dist_to_focus = 10.;
    let aperture = 0.;
    let vfov = 40.;

    Scene {
        geometry: BVH::new(
            rng,
            &mut vec![
                flip_normals(YZRect::new(0., 555., 0., 555., 555., red.clone())),
                Box::new(YZRect::new(0., 555., 0., 555., 0., green.clone())),
                Box::new(XZRect::new(213., 343., 227., 332., 554., light.clone())),
                flip_normals(XZRect::new(0., 555., 0., 555., 555., white.clone())),
                Box::new(XZRect::new(0., 555., 0., 555., 0., white.clone())),
                flip_normals(XYRect::new(0., 555., 0., 555., 555., white.clone())),
                Box::new(Translate::new(
                    RotateY::new(
                        CornellBox::new((0, 0, 0), (165, 165, 165), white.clone()),
                        -18.,
                    ),
                    (130, 0, 65),
                )),
                Box::new(Translate::new(
                    RotateY::new(
                        CornellBox::new((0, 0, 0), (165, 330, 165), white.clone()),
                        15.,
                    ),
                    (265, 0, 295),
                )),
            ],
            0.,
            1.,
        ),
        camera: Camera::new(
            lookfrom,
            lookat,
            Vec3::new(0., 1., 0.),
            vfov,
            config.aspect_ratio,
            aperture,
            dist_to_focus,
            0.,
            1.,
        ),
        use_ambient_light: false,
    }
}

pub fn cornell_fog(config: &Config, rng: &mut impl Rng) -> Scene {
    let red = Material::Diffuse(Texture::solid((0.65, 0.05, 0.05)));
    let white = Material::Diffuse(Texture::solid((0.73, 0.73, 0.73)));
    let green = Material::Diffuse(Texture::solid((0.12, 0.45, 0.15)));
    let light = Material::Light(Texture::solid((7., 7., 7.)));

    let lookfrom = Vec3::new(278., 278., -800.);
    let lookat = Vec3::new(278., 278., 0.);
    let dist_to_focus = 10.;
    let aperture = 0.;
    let vfov = 40.;

    Scene {
        geometry: BVH::new(
            rng,
            &mut vec![
                flip_normals(YZRect::new(0., 555., 0., 555., 555., green.clone())),
                Box::new(YZRect::new(0., 555., 0., 555., 0., red.clone())),
                Box::new(XZRect::new(113., 443., 127., 432., 554., light.clone())),
                flip_normals(XZRect::new(0., 555., 0., 555., 555., white.clone())),
                Box::new(XZRect::new(0., 555., 0., 555., 0., white.clone())),
                flip_normals(XYRect::new(0., 555., 0., 555., 555., white.clone())),
                Box::new(ConstantMedium::new(
                    Translate::new(
                        RotateY::new(
                            CornellBox::new((0, 0, 0), (165, 165, 165), white.clone()),
                            -18.,
                        ),
                        (130, 0, 65),
                    ),
                    0.01,
                    Texture::solid((1., 1., 1.)),
                )),
                Box::new(ConstantMedium::new(
                    Translate::new(
                        RotateY::new(
                            CornellBox::new((0, 0, 0), (165, 330, 165), white.clone()),
                            15.,
                        ),
                        (265, 0, 295),
                    ),
                    0.01,
                    Texture::solid((0., 0., 0.)),
                )),
            ],
            0.,
            1.,
        ),
        camera: Camera::new(
            lookfrom,
            lookat,
            Vec3::new(0., 1., 0.),
            vfov,
            config.aspect_ratio,
            aperture,
            dist_to_focus,
            0.,
            1.,
        ),
        use_ambient_light: false,
    }
}
