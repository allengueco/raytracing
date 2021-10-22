use image::{Rgb, RgbImage, ImageBuffer};
use image::imageops;
use nalgebra::{Vector3, vector, UnitVector3, Unit};
use num_traits::{Float, Num};
use crate::ray::Ray;

fn from_vec2(v: Vector3<u8>) -> Rgb<u8> {
    Rgb([v.x, v.y, v.z])
}

fn first(width: u32, height: u32) -> RgbImage {
    let mut img: RgbImage = ImageBuffer::new(width, height);
    println!("P3\n255\n{} {}", width, height);
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let [r, g, b] = [
            ((x as f32 / (width) as f32) * 255.999) as u8,
            ((y as f32 / (height) as f32) * 255.999) as u8,
            ((0.25) * 255.999) as u8,
        ];
        println!("{} {} {}", r,g,b);
        *pixel = Rgb([r,g,b]);
    }

    imageops::flip_vertical(&img)
}
mod ray {
    use nalgebra::{Vector3, Scalar, ClosedMul, ClosedAdd, ClosedSub, ClosedDiv, UnitVector3};
    use num_traits::float::Float;
    use std::ops::Mul;
    use std::fmt::Debug;

    #[derive(Clone, Copy)]
    pub struct Ray<T: Scalar + Float + ClosedMul + ClosedAdd + ClosedSub + ClosedDiv> {
        pub origin: Vector3<T>,
        pub direction: Vector3<T>,
    }
    impl<T: Scalar + Float + ClosedMul + ClosedAdd + ClosedSub + ClosedDiv> Ray<T> {
        pub fn from(origin: Vector3<T>, direction: Vector3<T>) -> Self {
            Self {
                origin, direction
            }
        }

        pub fn at(self, t: T) -> Vector3<T> {
            self.origin + self.direction.component_mul(&Vector3::repeat(t))
        }
    }
}

fn ray_color<T: Num>(r: Ray<T>) -> Rgb<T> {
    let unit_vec = Unit::new_normalize(r.direction);
    let t = 0.5*(unit_vec.y + 1.0);
    (1. - t)*Rgb([1., 1., 1.]) + t*Rgb()
}
fn main() {
    use ray::Ray;
    use nalgebra::vector;

    const ASPECT_RATIO: f64 = 16. / 9.;
    const IMAGE_WIDTH: usize = 400;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH / ASPECT_RATIO) as usize;


    let ray = Ray::from(
        vector![1., 2., 3.],
        vector![3.,4.,5.]
    );

    ray.at(3.5);
}
