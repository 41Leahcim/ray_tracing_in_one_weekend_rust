use crate::{vec3::Point3, ray::Ray};

use super::{Hittable, HitRecord};

#[derive(Debug, Default)]
pub struct Sphere{
    center: Point3,
    radius: f64
}

impl Hittable for Sphere{
    fn hit(&self, ray: &Ray, time_min: f64, time_max: f64, record: &mut HitRecord) -> bool {
        let oc = ray.origin() - self.center;
        let a = ray.direction().length_squared();
        let half_b = oc.dot(ray.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0{
            return false;
        }
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if root < time_min || time_max < root{
            root = (-half_b + sqrtd) / a;
            if root < time_min || time_max < root{
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

impl Sphere{
    pub fn new(center: Point3, radius: f64) -> Self{
        Self { center, radius }
    }
}