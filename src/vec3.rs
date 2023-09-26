#![allow(dead_code)]
use std::{
    fmt,
    ops::{Add, AddAssign, Index, IndexMut, Mul, Neg, Sub},
};

#[derive(Debug, Default, Clone, Copy)]
pub struct Vec3(f64, f64, f64);

impl Vec3 {
    pub const fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Self(e0, e1, e2)
    }

    pub const fn x(self) -> f64 {
        self.0
    }

    pub const fn y(self) -> f64 {
        self.1
    }

    pub const fn z(self) -> f64 {
        self.2
    }

    pub fn mul_assign(&mut self, t: f64) {
        self.0 *= t;
        self.1 *= t;
        self.2 *= t;
    }

    pub fn div_assign(&mut self, t: f64) {
        self.0 /= t;
        self.1 /= t;
        self.2 /= t;
    }

    pub fn mul(self, t: f64) -> Self {
        Self(t * self.0, t * self.1, t * self.2)
    }

    pub fn div(self, t: f64) -> Self {
        Self(self.0 / t, self.1 / t, self.2 / t)
    }

    pub fn dot(self, other: Self) -> f64 {
        self.0
            .mul_add(other.0, self.1.mul_add(other.1, self.2 * other.2))
    }

    pub fn cross(self, other: Self) -> Self {
        Self(
            self.1.mul_add(other.2, -self.2 * other.1),
            self.2.mul_add(other.0, -self.0 * other.2),
            self.0.mul_add(other.1, -self.1 * other.0),
        )
    }

    pub fn unit_vector(self) -> Self {
        self.div(self.length())
    }

    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.0
            .mul_add(self.0, self.1.mul_add(self.1, self.2 * self.2))
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1, -self.2)
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            _ => panic!("Index out of bounds!"),
        }
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            _ => panic!("Index out of bounds!"),
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.0, self.1, self.2)
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

pub fn mul(t: f64, v: Vec3) -> Vec3 {
    Vec3(t * v.0, t * v.1, t * v.2)
}

impl std::iter::Sum for Vec3 {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut result = Self::default();
        for vec3 in iter {
            result += vec3;
        }
        result
    }
}

pub type Point3 = Vec3;
