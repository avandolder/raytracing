mod camera;
mod hittable;
mod ray;
mod sphere;
mod vec3;

use rand::prelude::*;

use camera::Camera;
use hittable::Hittable;
use ray::Ray;
use sphere::Sphere;
use vec3::Vec3;

fn random_f32() -> f32 {
    thread_rng().gen::<f32>()
}

fn random_in_unit_sphere() -> Vec3 {
    let mut p = Vec3(1., 1., 1.);
    while p.squared_length() >= 1. {
        p = 2. * Vec3(random_f32(), random_f32(), random_f32()) - Vec3(1., 1., 1.);
    }
    p
}

fn color(r: &Ray, world: &Vec<&dyn Hittable>) -> Vec3 {
    if let Some(rec) = world.hit(r, 0.001, std::f32::MAX) {
        let target = rec.p + rec.normal + random_in_unit_sphere();
        0.5 * color(&Ray::new(rec.p, target - rec.p), world)
    } else {
        let unit_direction = r.direction().unit_vector();
        let t = 0.5 * (unit_direction.y() + 1.);
        (1. - t) * Vec3(1., 1., 1.) + t * Vec3(0.5, 0.7, 1.)
    }
}

fn main() {
    let nx = 200;
    let ny = 100;
    let ns = 100;
    println!("P3\n{} {}\n255", nx, ny);

    let spheres = [
        Sphere::new(Vec3(0., 0., -1.), 0.5),
        Sphere::new(Vec3(0., -100.5, -1.), 100.),
    ];
    let world: Vec<&dyn Hittable> = spheres.iter().map(|s| s as &dyn Hittable).collect();
    let cam = Camera::new();

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3::default();
            for _ in 0..ns {
                let u = (i as f32 + random_f32()) / nx as f32;
                let v = (j as f32 + random_f32()) / ny as f32;
                let r = cam.get_ray(u, v);
                col += color(&r, &world);
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
