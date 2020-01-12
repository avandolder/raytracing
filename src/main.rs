mod vec3;

use vec3::Vec3;

fn main() {
    let nx = 200;
    let ny = 100;
    println!("P3\n{} {}\n255", nx, ny);

    for j in (0..ny).rev() {
        for i in 0..nx {
            let color = Vec3(i as f32 / nx as f32, j as f32 / ny as f32, 0.2);
            let (ir, ig, ib) = (
                (255.99 * color.0) as i32,
                (255.99 * color.1) as i32,
                (255.99 * color.2) as i32,
            );
            println!("{} {} {}", ir, ig, ib);
        }
    }
}
