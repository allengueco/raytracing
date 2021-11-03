use std::ops::Range;

use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::Num;

pub struct World(pub Vec<Box<dyn Hittable>>);

impl Hittable for World {
    fn hit(&self, ray: Ray, range: Range<Num>) -> Option<HitRecord> {
        let mut hit_record: Option<HitRecord> = None;
        let mut closest = range.end;

        for h in &self.0 {
            if let Some(rec) = h.hit(ray, range.start..closest) {
                closest = rec.t;
                hit_record = Some(rec);
            }
        }
        hit_record
    }
}
