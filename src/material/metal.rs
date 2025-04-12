use crate::{ray::Ray, vec3::Color};

use super::Material;

#[derive(Debug, Default, Clone, Copy)]
pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub const fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        ray_in: &crate::ray::Ray,
        record: &crate::hittable::HitRecord,
    ) -> Option<(Color, crate::ray::Ray)> {
        let reflected = ray_in.direction().reflect(&record.normal());
        let scattered = Ray::new(record.point(), reflected);
        let attenuation = self.albedo;
        Some((attenuation, scattered))
    }
}
