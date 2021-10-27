#![allow(dead_code, unused_imports)]

use std::fs::File;
use std::io;
use std::io::Write;
use std::ops::Div;

use crate::camera::{Camera, Viewport};
use crate::hittable::{HitRecord, Hittable};
use crate::image::{AspectRatio, Image};
use crate::ray::Ray;
use crate::shapes::Sphere;
use crate::vec3::{Color, Point3, Vector3};
use crate::world::World;

mod ray;
mod vec3;
mod camera;
mod image;
mod hittable;
mod shapes;mod world;

pub(crate) type Num = f64;

pub(crate) fn write_color<W: io::Write>(writer: &mut W, pixel_color: Color, samples: usize) {
    let scale = 1.0 / samples as Num;
    let [mut r, mut g, mut b] = [
        pixel_color.x,
        pixel_color.y,
        pixel_color.z
    ];
    r *= scale;
    g *= scale;
    b *= scale;

    let r = (255.99 * r).max(0.).min(255.) as u8;
    let g = (255.99 * g).max(0.).min(255.) as u8;
    let b = (255.99 * b).max(0.).min(255.) as u8;

    writer
        .write_all(format!("{} {} {}\n", r, g, b).as_ref())
        .unwrap();
}

pub(crate) fn ray_color(ray: Ray, world: &World) -> Color {
    if let Some(rec) = world.hit(ray, 0.0..Num::INFINITY) {
        return 0.5 * (rec.normal + Color::from_elem(1.));
    }

    let unit_direction = ray.direction.normalize();
    let t = (unit_direction.y + 1.0) * 0.5;
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

pub(crate) fn render(world: &World, image: Image, camera: Camera, samples: usize) -> io::Result<
    ()> {
    let mut file = File::create("blended.ppm")?;
    file.write_all(format!("P3\n{} {}\n255\n", image.width, image.height).as_bytes())?;

    for y in (0..image.height).rev() {
        io::stderr().write_all(format!("\rScan lines remaining: {}", y).as_bytes())?;
        io::stderr().flush()?;
        for x in 0..image.width {
            let mut color = Color::from_elem(0.);
            for _ in 0..samples {
                let u = (x as Num + rand::random::<Num>()) / (image.width - 1) as Num;
                let v = (y as Num + rand::random::<Num>()) / (image.height - 1) as Num;
                let r = camera.ray(u, v);

                color += ray_color(r, world);
            }
            write_color(&mut file, color, samples);
        }
    }
    Ok(())
}

fn main() {
    //https://raytracing.github.io/books/RayTracingInOneWeekend.html

    // first(256, 256).unwrap();

    const SAMPLES: usize = 100;
    let aspect_ratio = AspectRatio(16. / 9.);
    let image = Image::from_width(aspect_ratio, 400);
    let camera = Camera::new(
        Viewport::from_height(aspect_ratio, 2),
        Point3::new(0., 0., 0.));

    let world = World(vec![
        Box::new(Sphere::new(Point3::new(0., 0., -1.), 0.5)),
        Box::new(Sphere::new(Point3::new(0., -100.5, -1.), 100.))]
    );
    render(&world, image, camera, SAMPLES).unwrap();
}
