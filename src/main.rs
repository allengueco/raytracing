#![allow(un)]
use std::fs::File;
use std::io;
use std::io::Write;

use crate::ray::Ray;
use crate::vec3::{Color, Point3, Vector3};

mod ray;
mod vec3;

mod camera {
    #[derive(Copy, Clone)]
    pub struct Camera {}
}

pub(crate) type Num = f64;

pub(crate) fn write_color<W: io::Write>(writer: &mut W, pixel_color: Color) {
    let [r, g, b] = [
        (pixel_color.x * 255.99) as u8,
        (pixel_color.y * 255.99) as u8,
        (pixel_color.z * 255.99) as u8,
    ];
    writer
        .write_all(format!("{} {} {}\n", r, g, b).as_ref())
        .unwrap();
}

pub(crate) fn ray_color(ray: Ray) -> Color {
    let unit_direction = ray.direction.normalize();
    let t: Num = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

pub(crate) fn first(width: usize, height: usize) -> io::Result<()> {
    let mut file = File::create("first.ppm")?;
    file.write_all(format!("P3\n{} {}\n255\n", width, height).as_bytes())?;
    for y in (0..height).rev() {
        for x in 0..width {
            let color = Color::new(
                (x as Num / (width - 1) as Num),
                (y as Num / (height - 1) as Num),
                0.25,
            );
            write_color(&mut file, color);
        }
    }
    Ok(())
}

fn main() {
    //https://raytracing.github.io/books/RayTracingInOneWeekend.html

    // Image
    const ASPECT_RATIO: f64 = 16. / 9.;
    const IMAGE_WIDTH: u64 = 256;
    const IMAGE_HEIGHT: u64 = 256;

    first(256, 256).unwrap();
}
