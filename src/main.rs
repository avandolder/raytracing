mod aabb;
mod bvh;
mod camera;
mod constant_medium;
mod cornellbox;
mod hittable;
mod material;
mod moving_sphere;
mod perlin;
mod ray;
mod rectangle;
mod rotate;
mod scene;
mod sphere;
mod texture;
mod translate;
mod vec3;

use std::{cell::LazyCell, fs::File, io, mem, slice};

use clap::{CommandFactory as _, Parser, ValueEnum};
use rand::Rng as _;
use rayon::{
    iter::{IndexedParallelIterator as _, ParallelIterator as _},
    slice::ParallelSliceMut as _,
};

use crate::{hittable::Hittable, ray::Ray, scene::Scene, vec3::Vec3};

const COLOR_CHANNELS: usize = 3;

#[derive(Clone, Debug)]
struct Image {
    data: Vec<f32>,
    width: usize,
    height: usize,
}

impl Image {
    fn new(width: usize, height: usize) -> Self {
        Self {
            data: vec![0.; width * height * COLOR_CHANNELS],
            width,
            height,
        }
    }
}

fn color(r: &Ray, world: &dyn Hittable, depth: i32, use_ambient_light: bool) -> Vec3 {
    if let Some(rec) = world.hit(r, 0.001, f32::MAX) {
        let emitted = rec.mat.emitted(rec.u, rec.v, rec.p);
        match rec.mat.scatter(r, &rec) {
            Some((attenuation, scattered)) if depth < 50 => {
                emitted + attenuation * color(&scattered, world, depth + 1, use_ambient_light)
            }
            _ => emitted,
        }
    } else if use_ambient_light {
        let unit_direction = r.direction().unit_vector();
        let t = 0.5 * (unit_direction.y() + 1.);
        (1. - t) * Vec3::new(1., 1., 1.) + t * Vec3::new(0.5, 0.7, 1.)
    } else {
        Vec3::new(0., 0., 0.)
    }
}

fn write_image_as_pfm(mut w: impl io::Write, image: &Image) -> io::Result<()> {
    writeln!(w, "PF")?;
    writeln!(w, "{} {}", image.width, image.height)?;

    // Endianness: negative number for little, positive for big
    #[cfg(target_endian = "little")]
    writeln!(w, "-1")?;
    #[cfg(target_endian = "big")]
    writeln!(w, "1")?;

    w.write_all(unsafe {
        slice::from_raw_parts(
            image.data.as_ptr() as *const u8,
            mem::size_of_val(image.data.as_slice()),
        )
    })
}

fn cast_more_rays(scene: &Scene, image: &mut Image, prev: u32, n: u32) {
    let (wf, hf, nf) = (image.width as f32, image.height as f32, n as f32);

    image
        .data
        .par_chunks_exact_mut(3)
        .enumerate()
        .for_each_init(rand::rng, |rng, (idx, px)| {
            let (x, y) = (idx % image.width, idx / image.height);
            let color = (0..n)
                .map(|_| {
                    let u = (x as f32 + rng.random::<f32>()) / wf;
                    let v = (y as f32 + rng.random::<f32>()) / hf;
                    color(
                        &scene.camera.get_ray(u, v),
                        &scene.geometry,
                        0,
                        scene.use_ambient_light,
                    )
                })
                .sum::<Vec3>();
            (0..3).for_each(|i| {
                px[i] = ((px[i] * px[i] * prev as f32 + color[i]) / (prev as f32 + nf)).sqrt()
            });
        });
}

fn progressive_cast(config: &Config, scene: &Scene) -> io::Result<Image> {
    let mut img = Image::new(config.width, config.height);

    let (mut i, mut step, mut so_far) = (0, 1, 0);
    while so_far < config.samples {
        cast_more_rays(scene, &mut img, so_far, (so_far + step).min(config.samples));

        write_image_as_pfm(File::create(format!("out-{:02}.pfm", i))?, &img)?;

        i += 1;
        so_far += step;
        step *= 2;
    }

    Ok(img)
}

thread_local! {
    static OIDN_DEVICE: LazyCell<oidn::Device> = LazyCell::new(oidn::Device::new);
}

fn denoise(image: &mut Image) {
    OIDN_DEVICE.with(|device| {
        oidn::RayTracing::new(device)
            .image_dimensions(image.width, image.height)
            .filter_quality(oidn::Quality::High)
            .filter_in_place(&mut image.data)
            .expect("failed to denoise")
    });
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
enum Scenes {
    Random,
    CornellBox,
    CornellFog,
}

#[derive(Debug, Parser)]
#[command(version, about, disable_help_flag = true)]
struct Options {
    #[arg(short, long, default_value_t = 1000)]
    width: usize,
    #[arg(short, long, default_value_t = 1000)]
    height: usize,

    #[arg(short, long)]
    progressive: bool,

    #[arg(short, long)]
    denoise: bool,

    #[arg(short, long, default_value_t = 50)]
    samples: u32,

    #[arg(short = '?', long)]
    help: bool,

    #[arg(long, value_enum, default_value_t = Scenes::Random)]
    scene: Scenes,
}

struct Config {
    width: usize,
    height: usize,
    aspect_ratio: f32,
    samples: u32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = Options::parse();
    if options.help {
        Options::command().print_help()?;
        return Ok(());
    }

    let config = Config {
        width: options.width,
        height: options.height,
        aspect_ratio: options.width as f32 / options.height as f32,
        samples: options.samples,
    };

    let scene = match options.scene {
        Scenes::Random => scene::random_scene(&config),
        Scenes::CornellBox => scene::cornell_box(&config),
        Scenes::CornellFog => scene::cornell_fog(&config),
    };

    let mut img = if options.progressive {
        progressive_cast(&config, &scene)?
    } else {
        let mut img = Image::new(config.width, config.height);
        cast_more_rays(&scene, &mut img, 0, config.samples);
        img
    };

    if options.denoise {
        denoise(&mut img);
    }

    write_image_as_pfm(File::create("out.pfm")?, &img)?;

    Ok(())
}
