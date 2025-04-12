use crate::{
    interval::Interval,
    ray::Ray,
    vec3::{Point3, Vec3},
};

pub mod list;
pub mod sphere;

pub struct HitRecord {
    point: Point3,
    normal: Vec3,
    time: f64,
    #[expect(dead_code)]
    front_face: bool,
}

impl HitRecord {
    pub fn new(ray: &Ray, outward_normal: Vec3, point: Point3, time: f64) -> Self {
        let front_face = ray.direction().dot(&outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        Self {
            point,
            normal,
            time,
            front_face,
        }
    }

    pub const fn point(&self) -> Point3 {
        self.point
    }

    pub const fn normal(&self) -> Vec3 {
        self.normal
    }

    pub const fn time(&self) -> f64 {
        self.time
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_time: Interval) -> Option<HitRecord>;
}
