mod aabb;
mod bvh;
mod camera;
mod cornellbox;
mod hittable;
mod material;
mod moving_sphere;
mod perlin;
mod ray;
mod rectangle;
mod sphere;
mod texture;
mod vec3;

use image::GenericImageView;
use rand::Rng;

use bvh::BVH;
use camera::Camera;
use cornellbox::CornellBox;
use hittable::{Hittable, flip_normals};
use material::Material;
use moving_sphere::MovingSphere;
use ray::Ray;
use rectangle::{XYRect, XZRect, YZRect};
use sphere::Sphere;
use texture::Texture;
use vec3::Vec3;

fn random_scene() -> Vec<Box<dyn Hittable>> {
    let n = 500;
    let mut rng = rand::thread_rng();
    let mut world: Vec<Box<dyn Hittable>> = Vec::with_capacity(n + 1);

    let checker = Texture::checker(
        Texture::solid((0.2, 0.3, 0.1)),
        Texture::solid((0.9, 0.9, 0.9)),
    );
    world.push(Box::new(Sphere::new(
        Vec3::new(0., -1000., 0.),
        1000.,
        Material::Diffuse(checker),
    )));

    for a in -11..11 {
        for b in -11..11 {
            let center = Vec3::new(
                a as f32 + 0.9 * rng.gen::<f32>(),
                0.2,
                b as f32 + 0.9 * rng.gen::<f32>(),
            );
            if (center - Vec3::new(4., 0.2, 0.)).length() <= 0.9 {
                continue;
            }

            let choose_mat = rng.gen::<f32>();
            if choose_mat < 0.8 {
                world.push(Box::new(MovingSphere::new(
                    center,
                    center + Vec3::new(0., 0.5 * rng.gen::<f32>(), 0.),
                    0.,
                    1.,
                    0.2,
                    Material::Diffuse(Texture::solid(Vec3::new(
                        rng.gen::<f32>() * rng.gen::<f32>(),
                        rng.gen::<f32>() * rng.gen::<f32>(),
                        rng.gen::<f32>() * rng.gen::<f32>(),
                    ))),
                )));
            } else if choose_mat < 0.95 {
                world.push(Box::new(Sphere::new(
                    center,
                    0.2,
                    Material::Metal(
                        Vec3::new(
                            0.5 * (1. + rng.gen::<f32>()),
                            0.5 * (1. + rng.gen::<f32>()),
                            0.5 * (1. + rng.gen::<f32>()),
                        ),
                        0.5 * rng.gen::<f32>(),
                    ),
                )));
            } else {
                world.push(Box::new(Sphere::new(center, 0.2, Material::Glass(1.5))));
            }
        }
    }

    world.push(Box::new(Sphere::new(
        Vec3::new(0., 1., 0.),
        1.,
        Material::Glass(1.5),
    )));

    let img = image::open("earthmap.jpg").unwrap();
    let data = img.raw_pixels();
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
    world
}

fn two_spheres() -> Vec<Box<dyn Hittable>> {
    let checker = Texture::checker(
        Texture::solid((0.2, 0.3, 0.1)),
        Texture::solid((0.9, 0.9, 0.9)),
    );
    vec![
        Box::new(Sphere::new(
            Vec3::new(0., -10., 0.),
            10.,
            Material::Diffuse(checker.clone()),
        )),
        Box::new(Sphere::new(
            Vec3::new(0., 10., 0.),
            10.,
            Material::Diffuse(checker.clone()),
        )),
    ]
}

fn two_perlin_spheres() -> Vec<Box<dyn Hittable>> {
    vec![
        Box::new(Sphere::new(
            Vec3::new(0., -1000., 0.),
            1000.,
            Material::Diffuse(Texture::noise(4.)),
        )),
        Box::new(Sphere::new(
            Vec3::new(0., 2., 0.),
            2.,
            Material::Diffuse(Texture::noise(4.)),
        )),
    ]
}

fn simple_light() -> Vec<Box<dyn Hittable>> {
    let pertext = Texture::noise(4.);
    let solidtext = Texture::solid((4., 4., 4.));
    vec![
        Box::new(Sphere::new(Vec3::new(0., -1000., 0.), 1000., Material::Diffuse(pertext.clone()))),
        Box::new(Sphere::new(Vec3::new(0., 2., 0.), 2., Material::Diffuse(pertext.clone()))),
        Box::new(Sphere::new(Vec3::new(0., 7., 0.), 2., Material::Light(solidtext.clone()))),
        Box::new(XYRect::new(3., 5., 1., 3., -2., Material::Light(solidtext.clone()))),
    ]
}

fn cornell_box() -> Vec<Box<dyn Hittable>> {
    let red = Material::Diffuse(Texture::solid((0.65, 0.05, 0.05)));
    let white = Material::Diffuse(Texture::solid((0.73, 0.73, 0.73)));
    let green = Material::Diffuse(Texture::solid((0.12, 0.45, 0.15)));
    let light = Material::Light(Texture::solid((15., 15., 15.)));

    vec![
        flip_normals(YZRect::new(0., 555., 0., 555., 555., green.clone())),
        Box::new(YZRect::new(0., 555., 0., 555., 0., red.clone())),
        Box::new(XZRect::new(213., 343., 227., 332., 554., light.clone())),
        flip_normals(XZRect::new(0., 555., 0., 555., 555., white.clone())),
        Box::new(XZRect::new(0., 555., 0., 555., 0., white.clone())),
        flip_normals(XYRect::new(0., 555., 0., 555., 555., white.clone())),
        Box::new(CornellBox::new((130, 0, 65), (295, 165, 230), white.clone())),
        Box::new(CornellBox::new((265, 0, 295), (430, 330, 460), white.clone())),
    ]
}

fn color(r: &Ray, world: &dyn Hittable, depth: i32) -> Vec3 {
    if let Some(rec) = world.hit(r, 0.001, std::f32::MAX) {
        let emitted = rec.mat.emitted(rec.u, rec.v, rec.p);
        match rec.mat.scatter(&r, &rec) {
            Some((attenuation, scattered)) if depth < 50 => {
                emitted + attenuation * color(&scattered, world, depth + 1)
            }
            _ => emitted,
        }
    } else {
        Vec3::new(0., 0., 0.)
    }
}

fn main() {
    let nx = 600;
    let ny = 400;
    let ns = 100;

    let world = BVH::new(&mut cornell_box(), 0., 1.);

    let lookfrom = Vec3::new(278., 278., -800.);
    let lookat = Vec3::new(278., 278., 0.);
    let dist_to_focus = 10.;
    let aperture = 0.;
    let vfov = 40.;
    let cam = Camera::new(
        lookfrom,
        lookat,
        Vec3::new(0., 1., 0.),
        vfov,
        (nx as f32) / (ny as f32),
        aperture,
        dist_to_focus,
        0.,
        1.,
    );

    let mut rng = rand::thread_rng();
    let mut imgbuf = image::ImageBuffer::new(nx, ny);

    for (i, j, pixel) in imgbuf.enumerate_pixels_mut() {
        let j = ny - j - 1; // Flip points vertically.
        let color = (0..ns).fold(Vec3::default(), |col, _| {
            let u = (i as f32 + rng.gen::<f32>()) / nx as f32;
            let v = (j as f32 + rng.gen::<f32>()) / ny as f32;
            col + color(&cam.get_ray(u, v), &world, 0)
        }) / ns as f32;
        let color = Vec3::new(color[0].sqrt(), color[1].sqrt(), color[2].sqrt());
        let color = Vec3::new(255.99, 255.99, 255.99) * color;

        *pixel = image::Rgb([color[0] as u8, color[1] as u8, color[2] as u8]);
    }

    imgbuf.save("out.png").unwrap();
}
