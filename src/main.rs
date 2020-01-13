mod camera;
mod hittable;
mod material;
mod ray;
mod sphere;
mod vec3;

use rand::prelude::*;

use camera::Camera;
use hittable::Hittable;
use material::Material;
use ray::Ray;
use sphere::Sphere;
use vec3::Vec3;

fn random_scene() -> Vec<Box<dyn Hittable>> {
    let n = 500;
    let mut rng = thread_rng();
    let mut world: Vec<Box<dyn Hittable>> = Vec::with_capacity(n + 1);
    world.push(Box::new(Sphere::new(
        Vec3(0., -1000., 0.),
        1000.,
        Material::Diffuse(Vec3(0.5, 0.5, 0.5)),
    )));

    for a in -11..11 {
        for b in -11..11 {
            let center = Vec3(
                a as f32 + 0.9 * rng.gen::<f32>(),
                0.2,
                b as f32 + 0.9 * rng.gen::<f32>(),
            );
            if (center - Vec3(4., 0.2, 0.)).length() <= 0.9 {
                continue;
            }

            let choose_mat = rng.gen::<f32>();
            if choose_mat < 0.8 {
                world.push(Box::new(Sphere::new(
                    center,
                    0.2,
                    Material::Diffuse(Vec3(
                        rng.gen::<f32>() * rng.gen::<f32>(),
                        rng.gen::<f32>() * rng.gen::<f32>(),
                        rng.gen::<f32>() * rng.gen::<f32>(),
                    )),
                )));
            } else if choose_mat < 0.95 {
                world.push(Box::new(Sphere::new(
                    center,
                    0.2,
                    Material::Metal(
                        Vec3(
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
        Vec3(0., 1., 0.),
        1.,
        Material::Glass(1.5),
    )));
    world.push(Box::new(Sphere::new(
        Vec3(-4., 1., 0.),
        1.,
        Material::Diffuse(Vec3(0.4, 0.2, 0.1)),
    )));
    world.push(Box::new(Sphere::new(
        Vec3(4., 1., 0.),
        1.,
        Material::Metal(Vec3(0.7, 0.6, 0.5), 0.),
    )));
    world
}

fn color(r: &Ray, world: &Vec<Box<dyn Hittable>>, depth: i32) -> Vec3 {
    if let Some(rec) = world.hit(r, 0.001, std::f32::MAX) {
        match rec.mat.scatter(&r, &rec) {
            Some((attenuation, scattered)) if depth < 50 => {
                attenuation * color(&scattered, world, depth + 1)
            }
            _ => Vec3(0., 0., 0.),
        }
    } else {
        let unit_direction = r.direction().unit_vector();
        let t = 0.5 * (unit_direction.y() + 1.);
        (1. - t) * Vec3(1., 1., 1.) + t * Vec3(0.5, 0.7, 1.)
    }
}

fn main() {
    let nx = 1200;
    let ny = 800;
    let ns = 10;
    println!("P3\n{} {}\n255", nx, ny);

    let world = random_scene();

    let lookfrom = Vec3(13., 2., 3.);
    let lookat = Vec3(0., 0., 0.);
    let dist_to_focus = 10.;
    let aperture = 0.1;
    let cam = Camera::new(
        lookfrom,
        lookat,
        Vec3(0., 1., 0.),
        20.,
        (nx as f32) / (ny as f32),
        aperture,
        dist_to_focus,
    );

    let mut rng = thread_rng();

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3::default();
            for _ in 0..ns {
                let u = (i as f32 + rng.gen::<f32>()) / nx as f32;
                let v = (j as f32 + rng.gen::<f32>()) / ny as f32;
                let r = cam.get_ray(u, v);
                col += color(&r, &world, 0);
            }
            col /= ns as f32;
            col = Vec3(col.0.sqrt(), col.1.sqrt(), col.2.sqrt());

            let (ir, ig, ib) = (
                (255.99 * col.0) as i32,
                (255.99 * col.1) as i32,
                (255.99 * col.2) as i32,
            );

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
