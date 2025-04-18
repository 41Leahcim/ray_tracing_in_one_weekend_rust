use crate::vec3::{Point3, Vec3};

#[derive(Debug, Default, Clone, Copy)]
pub struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl Ray {
    #[must_use]
    pub const fn new(origin: Point3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    #[must_use]
    pub const fn origin(&self) -> Point3 {
        self.origin
    }

    #[must_use]
    pub const fn direction(&self) -> Vec3 {
        self.direction
    }

    #[must_use]
    pub fn at(&self, time: f64) -> Point3 {
        self.origin + time * self.direction
    }
}
