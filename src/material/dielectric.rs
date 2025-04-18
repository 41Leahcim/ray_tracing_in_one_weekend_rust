use rand::random;

use crate::{hittable::HitRecord, ray::Ray, vec3::Color};

use super::Material;

#[derive(Debug, Clone, Copy)]
pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    #[must_use]
    pub const fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }

    fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        r0 *= r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
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
        let cos_theta = record.normal().dot(&-unit_direction).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let direction = if refraction_index * sin_theta > 1.0
            || Self::reflectance(cos_theta, refraction_index) > random()
        {
            unit_direction.reflect(&record.normal())
        } else {
            unit_direction.refract(&record.normal(), refraction_index)
        };
        let scattered = Ray::new(record.point(), direction);
        Some((attenuation, scattered))
    }
}
