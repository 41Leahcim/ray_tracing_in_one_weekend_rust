use crate::{
    ray::Ray,
    vec3::{Color, Vec3},
};

use super::Material;

#[derive(Debug, Default, Clone, Copy)]
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub const fn new(albedo: Color, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: fuzz.min(1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        ray_in: &crate::ray::Ray,
        record: &crate::hittable::HitRecord,
    ) -> Option<(Color, crate::ray::Ray)> {
        let reflected = ray_in.direction().reflect(&record.normal()).unit_vector()
            + (self.fuzz * Vec3::random_unit_vector());
        let scattered = Ray::new(record.point(), reflected);
        let attenuation = self.albedo;
        (scattered.direction().dot(&record.normal()) > 0.0).then_some((attenuation, scattered))
    }
}
