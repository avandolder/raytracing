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
mod rotate;
mod sphere;
mod texture;
mod translate;
mod vec3;

use std::{cell::LazyCell, fs::File, io, mem, slice};

use clap::{CommandFactory as _, Parser, ValueEnum};
use image::GenericImageView as _;
use rand::Rng as _;
use rayon::{
    iter::{IndexedParallelIterator as _, ParallelIterator as _},
    slice::ParallelSliceMut as _,
};

use bvh::BVH;
use camera::Camera;
use cornellbox::CornellBox;
use hittable::{Hittable, flip_normals};
use material::Material;
use moving_sphere::MovingSphere;
use ray::Ray;
use rectangle::{XYRect, XZRect, YZRect};
use rotate::RotateY;
use sphere::Sphere;
use texture::Texture;
use translate::Translate;
use vec3::Vec3;

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

struct Scene {
    geometry: BVH,
    camera: Camera,
    use_ambient_light: bool,
}

fn random_scene(aspect_ratio: f32) -> Scene {
    let n = 500;
    let mut rng = rand::rng();
    let mut world: Vec<Box<dyn Hittable + Sync>> = Vec::with_capacity(n + 1);

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
                a as f32 + 0.9 * rng.random::<f32>(),
                0.2,
                b as f32 + 0.9 * rng.random::<f32>(),
            );
            if (center - Vec3::new(4., 0.2, 0.)).length() <= 0.9 {
                continue;
            }

            let choose_mat = rng.random::<f32>();
            if choose_mat < 0.65 {
                world.push(Box::new(MovingSphere::new(
                    center,
                    center + Vec3::new(0., 0.5 * rng.random::<f32>(), 0.),
                    0.,
                    1.,
                    0.2,
                    Material::Diffuse(Texture::solid(Vec3::new(
                        rng.random::<f32>() * rng.random::<f32>(),
                        rng.random::<f32>() * rng.random::<f32>(),
                        rng.random::<f32>() * rng.random::<f32>(),
                    ))),
                )));
            } else if choose_mat < 0.8 {
                world.push(Box::new(Sphere::new(
                    center,
                    0.2,
                    Material::Diffuse(Texture::noise(4.)),
                )));
            } else if choose_mat < 0.95 {
                world.push(Box::new(Sphere::new(
                    center,
                    0.2,
                    Material::Metal(
                        Vec3::new(
                            0.5 * (1. + rng.random::<f32>()),
                            0.5 * (1. + rng.random::<f32>()),
                            0.5 * (1. + rng.random::<f32>()),
                        ),
                        0.5 * rng.random::<f32>(),
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
    let data = img.to_rgb8().into_raw();
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

    Scene {
        geometry: BVH::new(&mut world, 0., 1.),
        camera: Camera::new(
            Vec3::new(13., 2., 3.),
            Vec3::new(0., 0., 0.),
            Vec3::new(0., 1., 0.),
            20.,
            aspect_ratio,
            0.,
            10.,
            0.,
            1.,
        ),
        use_ambient_light: true,
    }
}

fn two_spheres() -> Vec<Box<dyn Hittable + Sync>> {
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

fn two_perlin_spheres() -> Vec<Box<dyn Hittable + Sync>> {
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

fn simple_light() -> Vec<Box<dyn Hittable + Sync>> {
    let pertext = Texture::noise(4.);
    let solidtext = Texture::solid((4., 4., 4.));
    vec![
        Box::new(Sphere::new(
            Vec3::new(0., -1000., 0.),
            1000.,
            Material::Diffuse(pertext.clone()),
        )),
        Box::new(Sphere::new(
            Vec3::new(0., 2., 0.),
            2.,
            Material::Diffuse(pertext.clone()),
        )),
        Box::new(Sphere::new(
            Vec3::new(0., 7., 0.),
            2.,
            Material::Light(solidtext.clone()),
        )),
        Box::new(XYRect::new(
            3.,
            5.,
            1.,
            3.,
            -2.,
            Material::Light(solidtext.clone()),
        )),
    ]
}

fn cornell_box(aspect_ratio: f32) -> Scene {
    let red = Material::Diffuse(Texture::solid((0.65, 0.05, 0.05)));
    let white = Material::Diffuse(Texture::solid((0.73, 0.73, 0.73)));
    let green = Material::Diffuse(Texture::solid((0.12, 0.45, 0.15)));
    let light = Material::Light(Texture::solid((15., 15., 15.)));

    let lookfrom = Vec3::new(278., 278., -800.);
    let lookat = Vec3::new(278., 278., 0.);
    let dist_to_focus = 10.;
    let aperture = 0.;
    let vfov = 40.;

    Scene {
        geometry: BVH::new(
            &mut vec![
                flip_normals(YZRect::new(0., 555., 0., 555., 555., red.clone())),
                Box::new(YZRect::new(0., 555., 0., 555., 0., green.clone())),
                Box::new(XZRect::new(213., 343., 227., 332., 554., light.clone())),
                flip_normals(XZRect::new(0., 555., 0., 555., 555., white.clone())),
                Box::new(XZRect::new(0., 555., 0., 555., 0., white.clone())),
                flip_normals(XYRect::new(0., 555., 0., 555., 555., white.clone())),
                Box::new(Translate::new(
                    RotateY::new(
                        CornellBox::new((0, 0, 0), (165, 165, 165), white.clone()),
                        -18.,
                    ),
                    (130, 0, 65),
                )),
                Box::new(Translate::new(
                    RotateY::new(
                        CornellBox::new((0, 0, 0), (165, 330, 165), white.clone()),
                        15.,
                    ),
                    (265, 0, 295),
                )),
            ],
            0.,
            1.,
        ),
        camera: Camera::new(
            lookfrom,
            lookat,
            Vec3::new(0., 1., 0.),
            vfov,
            aspect_ratio,
            aperture,
            dist_to_focus,
            0.,
            1.,
        ),
        use_ambient_light: false,
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
            image.data.len() * mem::size_of::<f32>(),
            // mem::size_of_val(&image.data),
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

fn progressive_cast(scene: &Scene, w: usize, h: usize, total_rays: u32) -> io::Result<Image> {
    let mut img = Image::new(w, h);

    let (mut i, mut step, mut so_far) = (0, 1, 0);
    while so_far < total_rays {
        cast_more_rays(scene, &mut img, so_far, (so_far + step).min(total_rays));

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
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = Options::parse();
    if options.help {
        Options::command().print_help()?;
        return Ok(());
    }

    let (width, height) = (options.width, options.height);
    let aspect_ratio = width as f32 / height as f32;

    let scene = match options.scene {
        Scenes::Random => random_scene(aspect_ratio),
        Scenes::CornellBox => cornell_box(aspect_ratio),
    };

    let mut img = if options.progressive {
        progressive_cast(&scene, width, height, options.samples)?
    } else {
        let mut img = Image::new(width, height);
        cast_more_rays(&scene, &mut img, 0, options.samples);
        img
    };

    if options.denoise {
        denoise(&mut img);
    }

    write_image_as_pfm(File::create("out.pfm")?, &img)?;

    Ok(())
}
