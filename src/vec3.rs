// lightweight class for vector3 class

use super::Num;
use std::ops;

#[derive(Clone, Copy, Debug)]
pub struct Vector3 {
    pub x: Num,
    pub y: Num,
    pub z: Num,
}

impl Vector3 {
    pub fn new(x: Num, y: Num, z: Num) -> Self {
        Self { x, y, z }
    }

    pub fn from_elem(e: Num) -> Self {
        Self { x: e, y: e, z: e }
    }

    pub fn length_squared(&self) -> Num {
        (self.x * self.x) + (self.y * self.y) + (self.z * self.z)
    }

    pub fn length(&self) -> Num {
        Num::sqrt(self.length_squared())
    }

    pub fn dot(self, other: Self) -> Num {
        self.x * other.x
            + self.y * other.y
            + self.z * other.z
    }

    pub fn cross(self, other: Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.y,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn normalize(self) -> Self {
        self / self.length()
    }
}

impl ops::AddAssign for Vector3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl ops::MulAssign for Vector3 {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl ops::DivAssign<Num> for Vector3 {
    fn div_assign(&mut self, rhs: Num) {
        *self = *self * (1. / rhs)
    }
}

// `vec3 + vec3`
impl ops::Add for Vector3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

// `vec3 - vec3`
impl ops::Sub for Vector3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

// `vec3 * vec3`
impl ops::Mul for Vector3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

// `vec3 * scalar`
impl ops::Mul<Num> for Vector3 {
    type Output = Self;

    fn mul(self, rhs: Num) -> Self::Output {
        Self {
            x: rhs * self.x,
            y: rhs * self.y,
            z: rhs * self.z,
        }
    }
}

// `scalar * vec`
impl ops::Mul<Vector3> for Num {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Self::Output {
        rhs * self
    }
}

// `-vec3`
impl ops::Neg for Vector3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

// `vec3 / scalar`
impl ops::Div<Num> for Vector3 {
    type Output = Self;

    fn div(self, rhs: Num) -> Self::Output {
        (1. / rhs) * self
    }
}

impl Default for Vector3 {
    fn default() -> Self {
        Self::from_elem(Num::default())
    }
}

pub(crate) type Color = Vector3;
pub(crate) type Point3 = Vector3;
