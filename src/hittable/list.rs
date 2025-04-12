use crate::interval::Interval;

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
    fn hit(&self, ray: &crate::ray::Ray, ray_time: Interval) -> Option<HitRecord> {
        let mut record = None;
        let mut closest_so_far = ray_time.max();

        for object in &self.objects {
            if let Some(temp_record) =
                object.hit(ray, Interval::new(ray_time.min(), closest_so_far))
            {
                closest_so_far = temp_record.time;
                record = Some(temp_record);
            }
        }
        record
    }
}
