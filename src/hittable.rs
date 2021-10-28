use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vector3, Color};
use crate::Num;
use std::ops::Range;

#[derive(Clone, Copy)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vector3,
    pub t: Num,
    pub mat: Material,
    front_face: bool,
}

impl Default for HitRecord {
    fn default() -> Self {
        Self {
            p: Point3::default(),
            normal: Vector3::default(),
            t: Num::default(),
            mat: Material::Lambertian { albedo: Color::default() },
            front_face: false,
        }
    }
}

impl HitRecord {
    pub(crate) fn new(ray: Ray, outward_normal: Vector3, p: Point3, t: Num, mat: Material) -> Self {
        let front_face = ray.direction.dot(outward_normal) < 0.;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        Self {
            p,
            normal,
            t,
            mat,
            front_face,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: Ray, range: Range<Num>) -> Option<HitRecord>;
}
