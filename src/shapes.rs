use crate::vec3::Point3;
use crate::Num;
use crate::hittable::{Hittable, HitRecord};
use crate::ray::Ray;
use std::ops::Range;

#[derive(Clone, Copy)]
pub struct Sphere {
    center: Point3,
    radius: Num
}

impl Sphere {
    pub(crate) fn new(center: Point3, radius: Num) -> Self {
        Self {
            center,
            radius
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, range: Range<Num>) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius*self.radius;

        let discriminant = half_b*half_b - a*c;
        if discriminant < 0. { return None }

        let sqrt_disc = Num::sqrt(discriminant);

        let mut root = (-half_b - sqrt_disc) / a;
        if !range.contains(&root) {
            root = (-half_b + sqrt_disc) / a;
            if !range.contains(&root) {
                return None
            }
        }

        let p = ray.at(root);
        let outward_normal = (p - self.center) / self.radius;

        Some(HitRecord::new(ray, outward_normal, p, root))
    }
}
