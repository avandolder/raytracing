fn main() {
    let nx = 200;
    let ny = 100;
    println!("P3\n{} {}\n255", nx, ny);

    for j in (0..ny).rev() {
        for i in 0..nx {
            let (r, g, b) = (i as f32 / nx as f32, j as f32 / ny as f32, 0.2);
            let (ir, ig, ib) = (
                (255.99 * r) as i32,
                (255.99 * g) as i32,
                (255.99 * b) as i32,
            );
            println!("{} {} {}", ir, ig, ib);
        }
    }
}
