use crate::vec3::Vector3;

use super::Num;

#[derive(Clone, Copy)]
pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3,
    pub time: Num,
}

impl Ray {
    pub fn from(origin: Vector3, direction: Vector3, time: Num) -> Self {
        Self {
            origin,
            direction,
            time,
        }
    }

    pub fn at(self, t: Num) -> Vector3 {
        self.origin + self.direction * t
    }
}
