use std::fmt::Debug;

use crate::{
    ray::Ray,
    vec3::{Point3, Vec3},
};

#[allow(clippy::module_name_repetitions)]
pub mod hittable_list;
pub mod sphere;

#[derive(Debug, Default, Clone, Copy)]
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    time: f64,
    font_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.font_face = ray.direction().dot(*outward_normal) < 0.0;
        self.normal = if self.font_face {
            *outward_normal
        } else {
            -*outward_normal
        };
    }
}

pub trait Hittable: Debug {
    fn hit(&self, ray: &Ray, time_min: f64, time_max: f64, record: &mut HitRecord) -> bool;
}
