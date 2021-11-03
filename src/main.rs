#![allow(dead_code, unused_imports)]

use std::fs::File;
use std::io;
use std::io::Write;
use std::ops::Div;

use rand::Rng;

use crate::camera::{Camera, Viewport};
use crate::hittable::{HitRecord, Hittable};
use crate::image::{AspectRatio, Image};
use crate::material::Material;
use crate::ray::Ray;
use crate::shapes::Sphere;
use crate::vec3::{Color, Point3, Vector3};
use crate::world::World;

mod camera;
mod hittable;
mod image;
mod material;
mod ray;
mod shapes;
mod vec3;
mod world;

pub(crate) type Num = f64;

pub(crate) fn write_color<W: io::Write>(writer: &mut W, pixel_color: Color, samples: usize) {
    let scale = 1.0 / (samples as Num);
    let [r, g, b] = [
        (pixel_color.x * scale).sqrt(),
        (pixel_color.y * scale).sqrt(),
        (pixel_color.z * scale).sqrt(),
    ];

    let ir = (255.99 * r.clamp(0.0,0.99)) as u8;
    let ig = (255.99 * g.clamp(0.0,0.99)) as u8;
    let ib = (255.99 * b.clamp(0.0,0.99)) as u8;

    writer
        .write_all(format!("{} {} {}\n", ir, ig, ib).as_ref())
        .unwrap();
}

pub(crate) fn ray_color<R: Rng>(ray: Ray, world: &World, depth: usize, rng: &mut R) -> Color {
    if depth <= 0 {
        return Color::zeros();
    }
    if let Some(rec) = world.hit(ray, 0.01..Num::MAX) {
        if let Some((attenuation, scattered)) = rec.mat.scatter(ray, rec, rng) {
            return attenuation * ray_color(scattered, world, depth-1, rng);
        }
        return Color::zeros();
    }

    // Blue to white gradient if the ray does not hit anything
    let unit_direction = ray.direction.normalize();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

pub(crate) fn render<R: Rng>(
    world: &World,
    image: Image,
    camera: Camera,
    samples: usize,
    depth: usize,
    rng: &mut R,
) -> io::Result<()> {
    let mut file = File::create("blended.ppm")?;
    eprintln!("{}x{}", image.width, image.height);
    file.write_all(format!("P3\n{} {}\n255\n", image.width, image.height).as_bytes())?;

    for y in (0..image.height).rev() {
        io::stderr().write_all(format!("\rScan lines remaining: {}", y).as_bytes())?;
        io::stderr().flush()?;
        for x in 0..image.width {
            let mut pixel_color = Color::zeros();
            for _ in 0..samples {
                let u = (x as Num + rng.gen::<Num>()) / (image.width - 1) as Num;
                let v = (y as Num + rng.gen::<Num>()) / (image.height - 1) as Num;
                let r = camera.cast_ray(u, v);

                pixel_color += ray_color(r, world, depth, rng);
            }
            write_color(&mut file, pixel_color, samples);
        }
    }
    Ok(())
}

fn main() {
    //https://raytracing.github.io/books/RayTracingInOneWeekend.html

    // first(256, 256).unwrap();

    const SAMPLES: usize = 50;
    const MAX_DEPTH: usize = 10;
    let aspect_ratio = AspectRatio(16. / 9.);
    let image = Image::from_width(aspect_ratio, 400);
    let camera = Camera::new(
        Viewport::from_height(aspect_ratio, 2),
        Point3::zeros()
    );
    let mut rng = rand::thread_rng();

    let world = World(vec![
        Box::new(Sphere::new(
            Point3::new(0., -100.5, -1.),
            100.,
            Material::Lambertian {
                albedo: Color::new(0.8, 0.8, 0.),
            },
        )),
        Box::new(Sphere::new(
            Point3::new(0., 0., -1.),
            0.5,
            Material::Lambertian {
                albedo: Color::new(0.7, 0.3, 0.3),
            },
        )),
        Box::new(Sphere::new(
            Point3::new(-1., 0., -1.),
            0.5,
            Material::Metal {
                albedo: Color::new(0.8, 0.8, 0.8),
            },
        )),
        Box::new(Sphere::new(
            Point3::new(1., 0., -1.),
            0.5,
            Material::Metal {
                albedo: Color::new(0.8, 0.6, 0.2),
            },
        )),
    ]);
    let _simple_world = World(vec![
        Box::new(Sphere::new(
            Point3::new(0., 0., -1.),
            0.5,
            Material::Lambertian {
                albedo: Color::new(0.8, 0.8, 0.),
            },
        )),
        Box::new(Sphere::new(
            Point3::new(0., -100.5, -1.),
            100.,
            Material::Lambertian {
                albedo: Color::new(0.8, 0.8, 0.),
            },
        )),
    ]);
    render(&world, image, camera, SAMPLES, MAX_DEPTH, &mut rng).unwrap();
}
