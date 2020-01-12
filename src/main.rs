mod ray;
mod vec3;

use ray::Ray;
use vec3::Vec3;

fn color(r: &Ray) -> Vec3 {
    let unit_direction = r.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.);
    Vec3(1., 1., 1.)*(1. - t) + Vec3(0.5, 0.7, 1.)*t
}

fn main() {
    let nx = 200;
    let ny = 100;
    println!("P3\n{} {}\n255", nx, ny);

    let lower_left_corner = Vec3(-2., -1., -1.);
    let horizontal = Vec3(4., 0., 0.);
    let vertical = Vec3(0., 2., 0.);
    let origin = Vec3(0., 0., 0.);

    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = i as f32 / nx as f32;
            let v = j as f32 / ny as f32;
            let r = Ray::new(origin, lower_left_corner + horizontal*u + vertical*v);
            let col = color(&r);
            let (ir, ig, ib) = (
                (255.99 * col.0) as i32,
                (255.99 * col.1) as i32,
                (255.99 * col.2) as i32,
            );
            println!("{} {} {}", ir, ig, ib);
        }
    }
}
