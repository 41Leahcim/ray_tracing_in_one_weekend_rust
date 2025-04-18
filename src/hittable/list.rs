use crate::{interval::Interval, ray::Ray};

use super::{HitRecord, Hittable};

#[derive(Default)]
pub struct HittableList {
    objects: Vec<Box<dyn Hittable + Sync>>,
}

impl HittableList {
    pub const fn new(objects: Vec<Box<dyn Hittable + Sync>>) -> Self {
        Self { objects }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Box<dyn Hittable + Sync>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, ray_time: Interval) -> Option<HitRecord> {
        self.objects
            .iter()
            .filter_map(|object| object.hit(ray, ray_time))
            .min_by(|left, right| left.time.total_cmp(&right.time))
    }
}
