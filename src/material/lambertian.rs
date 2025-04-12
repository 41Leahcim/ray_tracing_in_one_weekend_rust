use crate::{
    ray::Ray,
    vec3::{Color, Vec3},
};

use super::Material;

#[derive(Debug, Clone, Copy)]
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub const fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _ray_in: &crate::ray::Ray,
        record: &crate::hittable::HitRecord,
    ) -> Option<(Color, crate::ray::Ray)> {
        let mut scatter_direction = record.normal() + Vec3::random_unit_vector();

        // Catch degenerate scatter_direction
        if scatter_direction.near_zero() {
            scatter_direction = record.normal();
        }

        let scattered = Ray::new(record.point(), scatter_direction);
        let attenuation = self.albedo;
        Some((attenuation, scattered))
    }
}
