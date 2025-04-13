use crate::{hittable::HitRecord, ray::Ray, vec3::Color};

use super::Material;

#[derive(Debug, Clone, Copy)]
pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub const fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, record: &HitRecord) -> Option<(Color, Ray)> {
        let attenuation = Color::new([1.0; 3]);
        let refraction_index = if record.front_face() {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };
        let unit_direction = ray_in.direction().unit_vector();
        let refracted = unit_direction.refract(&record.normal(), refraction_index);
        let scattered = Ray::new(record.point(), refracted);
        Some((attenuation, scattered))
    }
}
