use super::{HitRecord, Hittable};

#[derive(Default)]
pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub const fn new(objects: Vec<Box<dyn Hittable>>) -> Self {
        Self { objects }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(
        &self,
        ray: &crate::ray::Ray,
        ray_time_min: f64,
        ray_time_max: f64,
    ) -> Option<HitRecord> {
        self.objects
            .iter()
            .filter_map(|object| {
                object
                    .hit(ray, ray_time_min, ray_time_max)
                    .map(|record| (record.time, record))
            })
            .min_by(|(time1, _), (time2, _)| time1.total_cmp(time2))
            .map(|(_, record)| record)
    }
}
