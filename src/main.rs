mod aabb;
mod bvh;
mod camera;
mod hittable;
mod material;
mod moving_sphere;
mod ray;
mod sphere;
mod texture;
mod vec3;

use rand::prelude::*;

use bvh::BVH;
use camera::Camera;
use hittable::Hittable;
use material::Material;
use moving_sphere::MovingSphere;
use ray::Ray;
use sphere::Sphere;
use texture::Texture;
use vec3::Vec3;

fn random_scene() -> Vec<Box<dyn Hittable>> {
    let n = 500;
    let mut rng = thread_rng();
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
    world.push(Box::new(Sphere::new(
        Vec3::new(-4., 1., 0.),
        1.,
        Material::Diffuse(Texture::solid(Vec3::new(0.4, 0.2, 0.1))),
    )));
    world.push(Box::new(Sphere::new(
        Vec3::new(4., 1., 0.),
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

fn color(r: &Ray, world: &dyn Hittable, depth: i32) -> Vec3 {
    if let Some(rec) = world.hit(r, 0.001, std::f32::MAX) {
        match rec.mat.scatter(&r, &rec) {
            Some((attenuation, scattered)) if depth < 50 => {
                attenuation * color(&scattered, world, depth + 1)
            }
            _ => Vec3::new(0., 0., 0.),
        }
    } else {
        let unit_direction = r.direction().unit_vector();
        let t = 0.5 * (unit_direction.y() + 1.);
        (1. - t) * Vec3::new(1., 1., 1.) + t * Vec3::new(0.5, 0.7, 1.)
    }
}

fn main() {
    let nx = 600;
    let ny = 400;
    let ns = 100;
    println!("P3\n{} {}\n255", nx, ny);

    let world = BVH::new(&mut two_spheres(), 0., 1.);

    let lookfrom = Vec3::new(13., 2., 3.);
    let lookat = Vec3::new(0., 0., 0.);
    let dist_to_focus = 10.;
    let aperture = 0.;
    let cam = Camera::new(
        lookfrom,
        lookat,
        Vec3::new(0., 1., 0.),
        20.,
        (nx as f32) / (ny as f32),
        aperture,
        dist_to_focus,
        0.,
        1.,
    );

    let mut rng = thread_rng();

    for j in (0..ny).rev() {
        for i in 0..nx {
            let color = (0..ns).fold(Vec3::default(), |col, _| {
                let u = (i as f32 + rng.gen::<f32>()) / nx as f32;
                let v = (j as f32 + rng.gen::<f32>()) / ny as f32;
                col + color(&cam.get_ray(u, v), &world, 0)
            }) / ns as f32;
            let color = Vec3::new(color[0].sqrt(), color[1].sqrt(), color[2].sqrt());
            let color = Vec3::new(255.99, 255.99, 255.99) * color;

            println!(
                "{} {} {}",
                color[0] as i32, color[1] as i32, color[2] as i32
            );
        }
    }
}
