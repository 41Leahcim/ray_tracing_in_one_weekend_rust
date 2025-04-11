use crate::vec3::{Point3, Vec3};

#[derive(Debug, Default, Clone, Copy)]
pub struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl Ray {
    pub const fn new(origin: Point3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub const fn origin(&self) -> Point3 {
        self.origin
    }

    pub const fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn at(&self, time: f64) -> Point3 {
        self.origin + time * self.direction
    }
}
