#![allow(dead_code)]
use crate::vec3::{Vec3, Point3, mul};

#[derive(Debug, Default)]
struct Ray{
    origin: Point3,
    direction: Vec3
}

impl Ray{
    pub fn new(origin: Point3, direction: Vec3) -> Self{
        Self { origin, direction }
    }

    pub const fn origin(&self) -> Point3{
        self.origin
    }

    pub const  fn direction(&self) -> Vec3{
        self.direction
    }

    pub fn at(&self, t: f64) -> Point3{
        self.origin + mul(t, self.direction)
    }
}
