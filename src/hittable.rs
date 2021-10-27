use crate::Num;
use crate::vec3::{Point3, Vector3};
use crate::ray::Ray;
use std::ops::Range;

#[derive(Clone, Copy)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vector3,
    pub t: Num,
    front_face: bool
}

impl Default for HitRecord {
    fn default() -> Self {
        Self {
            p: Point3::default(),
            normal: Vector3::default(),
            t: Num::default(),
            front_face: false
        }
    }
}

impl HitRecord {
    pub(crate) fn new(ray: Ray, outward_normal: Vector3, p: Point3, t: Num) -> Self {
        let front_face = ray.direction.dot(outward_normal) < 0.;
        let normal = if front_face { outward_normal } else { -outward_normal };
        Self {
            p,
            normal,
            t,
            front_face
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: Ray, range: Range<Num>) -> Option<HitRecord>;
}
