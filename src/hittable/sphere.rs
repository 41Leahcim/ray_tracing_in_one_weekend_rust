use crate::{interval::Interval, ray::Ray, vec3::Point3};

use super::{HitRecord, Hittable};

#[derive(Debug, Default)]
pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: Interval, record: &mut HitRecord) -> bool {
        let oc = ray.origin() - self.center;
        let a = ray.direction().length_squared();
        let half_b = oc.dot(ray.direction());
        let c = self.radius.mul_add(-self.radius, oc.length_squared());

        let discriminant = half_b.mul_add(half_b, -a * c);

        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (-half_b + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }

        record.time = root;
        record.point = ray.at(record.time);
        let outward_normal = (record.point - self.center).div(self.radius);
        record.set_face_normal(ray, &outward_normal);

        true
    }
}

impl Sphere {
    pub const fn new(center: Point3, radius: f64) -> Self {
        Self { center, radius }
    }
}
