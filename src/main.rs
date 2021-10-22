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
}

mod vec3 {
    // lightweight class for vector3 class
    use num_traits::Float;

    #[derive(Clone, Copy, Debug)]
    pub struct Vector3<T: Float> {
        pub x: T,
        pub y: T,
        pub z: T,
    }

    impl<T: Float> Vector3<T> {
        pub fn new(x: T, y: T, z: T) -> Self {
            Self { x, y, z }
        }

        pub fn length_squared(&self) -> T {
            (self.x * self.x) + (self.y * self.y) + (self.z * self.z)
        }

        pub fn length(&self) -> T {
            T::sqrt(self.length_squared())
        }

        pub fn dot(self, other: Self) -> Self {
            Self {
                x: self.x * other.x,
                y: self.y * other.y,
                z: self.z * other.z,
            }
        }

        pub fn to_unit(self) -> Self {
            self / self.length()
        }

        pub fn cross(self, other: Self) -> Self {
            Self {
                x: self.y * other.z - self.z * other.y,
                y: self.z * other.x - self.x * other.y,
                z: self.x * other.y - self.y * other.x,
            }
        }
    }

    use std::ops;
    use nalgebra::{ClosedMul, Vector};
    use std::process::Output;

    // `vec3 + vec3`
    impl<T: Float> ops::Add for Vector3<T> {
        type Output = Self;

        fn add(self, rhs: Self) -> Self::Output {
            Self {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
                z: self.z + rhs.z
            }
        }
    }

    // `vec3 - vec3`
    impl<T: Float> ops::Sub for Vector3<T> {
        type Output = Self;

        fn sub(self, rhs: Self) -> Self::Output {
            Self {
                x: self.x - rhs.x,
                y: self.y - rhs.y,
                z: self.z - rhs.z
            }
        }
    }

    // `vec3 * vec3`
    impl<T: Float> ops::Mul for Vector3<T> {
        type Output = Self;

        fn mul(self, rhs: Self) -> Self::Output {
            Self {
                x: self.x * rhs.x,
                y: self.y * rhs.y,
                z: self.z * rhs.z
            }
        }
    }

    // `vec3 * scalar`
    impl<T: Float> ops::Mul<T> for Vector3<T> {
        type Output = Self;

        fn mul(self, rhs: T) -> Self::Output {
            Self {
                x: rhs * self.x,
                y: rhs * self.y,
                z: rhs * self.z
            }
        }
    }

    // `scalar * vec3`
    impl<T: Float> ops::Mul<Vector3<T>> for T {
        type Output = Vector3<T>;

        fn mul(self, rhs: Vector3<T>) -> Self::Output {
            Self {
                x: rhs.x * self,
                y: rhs.y * self,
                z: rhs.z * self,
            }
        }
    }

    // `-vec3`
    impl<T: Float> ops::Neg for Vector3<T> {
        type Output = Self;

        fn neg(self) -> Self::Output {
            Self { x: -self.x, y: -self.y, z: -self.z }
        }
    }

    // `vec3 / scalar`
    impl<T: Float> ops::Div<T> for Vector3<T> {
        type Output = Self;

        fn div(self, rhs: T) -> Self::Output {
            Self {
                x: self.x / rhs,
                y: self.y / rhs,
                z: self.z / rhs
            }
        }
    }


}
fn main() {
    //https://raytracing.github.io/books/RayTracingInOneWeekend.html
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
