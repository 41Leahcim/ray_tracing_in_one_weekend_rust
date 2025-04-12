use crate::{interval::Interval, ray::Ray, vec3::Point3};

use super::{HitRecord, Hittable};

#[derive(Debug, Default, Clone, Copy)]
pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub const fn new(center: Point3, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_time: Interval) -> Option<HitRecord> {
        let origin_center = self.center - ray.origin();
        let a = ray.direction().length_squared();
        let h = ray.direction().dot(&origin_center);
        let c = origin_center.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrt_discriminant = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (h - sqrt_discriminant) / a;
        if !ray_time.surrounds(root) {
            root = (h + sqrt_discriminant) / a;
            if !ray_time.surrounds(root) {
                return None;
            }
        }

        let time = root;
        let point = ray.at(time);
        let outward_normal = (point - self.center) / self.radius;
        Some(HitRecord::new(ray, outward_normal, point, time))
    }
}
