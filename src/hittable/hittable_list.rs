use std::sync::Arc;

use super::{Hittable, HitRecord};

#[derive(Debug, Default)]
pub struct HittableList{
    objects: Vec<Arc<dyn Hittable>>
}

impl HittableList{
    pub fn new(object: Arc<dyn Hittable>) -> Self{
        Self { objects: vec![object] }
    }

    pub fn clear(&mut self){
        self.objects.clear();
    }

    pub fn add(&mut self, object: Arc<dyn Hittable>){
        self.objects.push(object);
    }
}

impl Hittable for HittableList{
    fn hit(&self, ray: &crate::ray::Ray, time_min: f64, time_max: f64, record: &mut super::HitRecord) -> bool {
        let mut temp_record = HitRecord::default();
        let mut closest_so_far = time_max;

        self.objects.iter().filter(|object|{
            if object.hit(ray, time_min, closest_so_far, &mut temp_record){
                closest_so_far = temp_record.time;
                *record = temp_record;
                true
            }else{
                false
            }
        }).count() > 0
    }
}
