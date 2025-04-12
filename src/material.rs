use std::fmt::Debug;

use crate::{hittable::HitRecord, ray::Ray, vec3::Color};

pub mod lambertian;
pub mod metal;

pub trait Material: Debug + Sync + Send {
    fn scatter(&self, ray_in: &Ray, record: &HitRecord) -> Option<(Color, Ray)>;
}
