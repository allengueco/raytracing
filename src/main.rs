#![allow(dead_code, unused_imports)]

use std::fs::File;
use std::io;
use std::io::Write;
use std::ops::Div;

use rand::Rng;
use rayon::prelude::*;

use crate::camera::{Camera, Viewport};
use crate::hittable::{HitRecord, Hittable};
use crate::image::Image;
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

pub(crate) type Num = f32;

pub(crate) fn write_color<W: io::Write>(writer: &mut W, pixel_color: Color, samples: usize) {
    let scale = 1.0 / (samples as Num);
    let [r, g, b] = [
        (pixel_color.x * scale).sqrt(),
        (pixel_color.y * scale).sqrt(),
        (pixel_color.z * scale).sqrt(),
    ];

    let ir = (255.99 * r.clamp(0.0, 0.99)) as u8;
    let ig = (255.99 * g.clamp(0.0, 0.99)) as u8;
    let ib = (255.99 * b.clamp(0.0, 0.99)) as u8;

    writer
        .write_all(format!("{} {} {}\n", ir, ig, ib).as_ref())
        .unwrap();
}

pub(crate) fn ray_color<R: Rng>(ray: Ray, world: &World, depth: usize, rng: &mut R) -> Color {
    if depth <= 0 {
        return Color::zeros();
    }
    if let Some(rec) = world.hit(ray, 0.0001..Num::MAX) {
        if let Some((attenuation, scattered)) = rec.mat.scatter(ray, rec, rng) {
            return attenuation * ray_color(scattered, world, depth - 1, rng);
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
    let mut file = File::create("final_scene.ppm")?;
    eprintln!("{}x{}", image.width, image.height);
    file.write_all(format!("P3\n{} {}\n255\n", image.width, image.height).as_bytes())?;

    for j in (0..image.height).rev() {
        io::stderr().write_all(format!("\rScan lines remaining: {}", j).as_bytes())?;
        io::stderr().flush()?;
        for i in 0..image.width {
            let mut pixel_color = Color::zeros();
            for _ in 0..samples {
                let u = (i as Num + rng.gen::<Num>()) / (image.width - 1) as Num;
                let v = (j as Num + rng.gen::<Num>()) / (image.height - 1) as Num;
                let r = camera.cast_ray(u, v);
                pixel_color += ray_color(r, world, depth, rng);
            }
            write_color(&mut file, pixel_color, samples);
        }
    }
    io::stderr().write_all(format!("\nDone.\n").as_bytes())?;
    Ok(())
}

fn main() {
    //https://raytracing.github.io/books/RayTracingInOneWeekend.html

    const SAMPLES: usize = 500;
    const MAX_DEPTH: usize = 50;
    let image = Image::from_width(Camera::ASPECT_RATIO, 1200);
    let camera = Camera::new(
        Point3::new(13., 2., 3.),
        Point3::new(0., 0., 0.),
        Vector3::new(0., 1., 0.),
        20.0,
    );
    let mut rng = rand::thread_rng();

    let world = World(vec![
        // Ground
        Box::new(Sphere::new(
            Point3::new(0., -100.5, -1.),
            100.,
            Material::Lambertian {
                albedo: Color::new(0.8, 0.8, 0.),
            },
        )),
        //Left
        Box::new(Sphere::new(
            Point3::new(-1., 0., -1.),
            0.5,
            Material::Dielectric { ir: 1.5 },
        )),
        // Inner left
        Box::new(Sphere::new(
            Point3::new(-1., 0., -1.),
            -0.4,
            Material::Dielectric { ir: 1.5 },
        )),
        // Center
        Box::new(Sphere::new(
            Point3::new(0., 0., -1.),
            0.5,
            Material::Lambertian {
                albedo: Color::new(0.1, 0.2, 0.5),
            },
        )),
        //Right
        Box::new(Sphere::new(
            Point3::new(1., 0., -1.),
            0.5,
            Material::Metal {
                albedo: Color::new(0.8, 0.6, 0.2),
                fuzz: 0.0,
            },
        )),
    ]);
    let random_scene = final_scene(&mut rng);
    render(&random_scene, image, camera, SAMPLES, MAX_DEPTH, &mut rng).unwrap();
}

fn final_scene<R: Rng>(rng: &mut R) -> World {
    let mut world = World(vec![]);
    let ground = Box::new(Sphere::new(
        Point3::new(0., -1000., 0.),
        1000.,
        Material::Lambertian {
            albedo: Color::from_elem(0.5),
        },
    ));

    world.add(ground);

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen::<Num>();
            let center = Point3::new(
                a as Num + 0.9 * rng.gen::<Num>(),
                0.2,
                b as Num + 0.9 * rng.gen::<Num>(),
            );

            if (center - Point3::new(4., 0.2, 0.)).length() > 0.9 {
                match choose_mat {
                    c if c < 0.8 => {
                        let albedo = Color::random(rng) * Color::random(rng);
                        world.add(Box::new(Sphere::new(
                            center,
                            0.2,
                            Material::Lambertian { albedo },
                        )));
                    }
                    c if c < 0.95 => {
                        let albedo = Color::random_double(0.5..1., rng);
                        let fuzz = rng.gen_range(0. ..0.5);
                        world.add(Box::new(Sphere::new(
                            center,
                            0.2,
                            Material::Metal { albedo, fuzz },
                        )));
                    }
                    _ => {
                        world.add(Box::new(Sphere::new(
                            center,
                            0.2,
                            Material::Dielectric { ir: 1.5 },
                        )));
                    }
                }
            }
        }
    }
    world.add(Box::new(Sphere::new(
        Point3::new(0., 1., 0.),
        1.,
        Material::Dielectric { ir: 1.5 },
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(-4., 1., 0.),
        1.,
        Material::Lambertian {
            albedo: Color::new(0.4, 0.2, 0.1),
        },
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(4., 1., 0.),
        1.,
        Material::Metal {
            albedo: Color::new(0.7, 0.6, 0.5),
            fuzz: 0.0,
        },
    )));

    world
}
