use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::Point3;
use crate::Num;
use std::ops::Range;
use crate::material::Material;

#[derive(Clone, Copy)]
pub struct Sphere {
    center: Point3,
    radius: Num,
    mat: Material
}

impl Sphere {
    pub(crate) fn new(center: Point3, radius: Num, mat: Material) -> Self {
        Self { center, radius, mat }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, range: Range<Num>) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0. {
            return None;
        }

        let sqrt_disc = discriminant.sqrt();

        let mut root = (-half_b - sqrt_disc) / a;
        if !range.contains(&root) {
            root = (-half_b + sqrt_disc) / a;
            if !range.contains(&root) {
                return None
            }
        }

        let p = ray.at(root);
        let outward_normal = (p - self.center) / self.radius;

        Some(HitRecord::new(ray, outward_normal, p, root, self.mat))
    }
}
