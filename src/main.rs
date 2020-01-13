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

fn color(r: &Ray, world: &Vec<&dyn Hittable>, depth: i32) -> Vec3 {
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
    let nx = 200;
    let ny = 100;
    let ns = 100;
    println!("P3\n{} {}\n255", nx, ny);

    #[rustfmt::skip]
    let spheres = [
        Sphere::new(Vec3(0., 0., -1.), 0.5, Material::Diffuse(Vec3(0.1, 0.2, 0.5))),
        Sphere::new(Vec3(0., -100.5, -1.), 100., Material::Diffuse(Vec3(0.8, 0.8, 0.))),
        Sphere::new(Vec3(1., 0., -1.), 0.5, Material::Metal(Vec3(0.8, 0.6, 0.2), 0.)),
        Sphere::new(Vec3(-1., 0., -1.), 0.5, Material::Dielectric(1.5)),
        Sphere::new(Vec3(-1., 0., -1.), -0.45, Material::Dielectric(1.5)),
    ];
    let world: Vec<&dyn Hittable> = spheres.iter().map(|s| s as &dyn Hittable).collect();
    let cam = Camera::new();
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
