use std::ops::Range;

use crate::hittable::{HitRecord, Hittable};
use crate::Num;
use crate::ray::Ray;

pub struct World(pub Vec<Box<dyn Hittable>>);

impl Hittable for World {
    fn hit(&self, ray: Ray, range: Range<Num>) -> Option<HitRecord> {
        let mut hit_record: HitRecord = HitRecord::default();
        let mut closest = range.end;
        let mut hit = false;

        for h in &self.0 {
            if let Some(rec) = h.hit(ray, 0.0..closest) {
                hit = true;
                closest = rec.t;
                hit_record = rec;
            }
        }

        if hit { Some(hit_record) } else { None }
    }
}
