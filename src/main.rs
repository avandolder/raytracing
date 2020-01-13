mod hittable;
mod ray;
mod sphere;
mod vec3;

use hittable::Hittable;
use ray::Ray;
use sphere::Sphere;
use vec3::Vec3;

fn color(r: &Ray, world: &Vec<&dyn Hittable>) -> Vec3 {
    if let Some(rec) = world.hit(r, 0.0, std::f32::MAX) {
        0.5 * Vec3(
            rec.normal.x() + 1.,
            rec.normal.y() + 1.,
            rec.normal.z() + 1.,
        )
    } else {
        let unit_direction = r.direction().unit_vector();
        let t = 0.5 * (unit_direction.y() + 1.);
        (1. - t) * Vec3(1., 1., 1.) + t * Vec3(0.5, 0.7, 1.)
    }
}

fn main() {
    let nx = 200;
    let ny = 100;
    println!("P3\n{} {}\n255", nx, ny);

    let lower_left_corner = Vec3(-2., -1., -1.);
    let horizontal = Vec3(4., 0., 0.);
    let vertical = Vec3(0., 2., 0.);
    let origin = Vec3(0., 0., 0.);

    let spheres = [
        Sphere::new(Vec3(0., 0., -1.), 0.5),
        Sphere::new(Vec3(0., -100.5, -1.), 100.),
    ];
    let world: Vec<&dyn Hittable> = spheres.iter().map(|s| s as &dyn Hittable).collect();

    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = i as f32 / nx as f32;
            let v = j as f32 / ny as f32;
            let r = Ray::new(origin, lower_left_corner + horizontal * u + vertical * v);

            let p = r.point_at_parameter(2.);
            let col = color(&r, &world);

            let (ir, ig, ib) = (
                (255.99 * col.0) as i32,
                (255.99 * col.1) as i32,
                (255.99 * col.2) as i32,
            );

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
