use crate::vec3::Vector3;
use std::fmt::Debug;
use std::ops::Mul;

use super::Num;

#[derive(Clone, Copy)]
pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3,
}

impl Ray {
    pub fn from(origin: Vector3, direction: Vector3) -> Self {
        Self { origin, direction }
    }

    pub fn at(self, t: Num) -> Vector3 {
        self.origin + self.direction * t
    }
}
