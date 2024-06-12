use core::{
    fmt::{self, Display, Formatter},
    ops,
};

use crate::vec3::Vec3;

#[derive(Debug, Default, Clone, Copy)]
pub struct Color(Vec3);

impl Color {
    pub const fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Self(Vec3::new(e0, e1, e2))
    }

    pub const fn x(&self) -> f64 {
        self.0.x()
    }

    pub const fn y(&self) -> f64 {
        self.0.y()
    }

    pub const fn z(&self) -> f64 {
        self.0.z()
    }

    pub fn length_squared(&self) -> f64 {
        self.0.length_squared()
    }

    pub fn length(&self) -> f64 {
        self.0.length()
    }

    pub fn dot(&self, v: &Self) -> f64 {
        self.0.dot(&v.0)
    }

    pub fn cross(&self, v: &Self) -> Self {
        Self(self.0.cross(&v.0))
    }

    pub fn unit_vector(&self) -> Self {
        Self(self.0.unit_vector())
    }
}

impl ops::Neg for Color {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl ops::Index<usize> for Color {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        self.0.index(index)
    }
}

impl ops::IndexMut<usize> for Color {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.0.index_mut(index)
    }
}

impl ops::AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl ops::MulAssign<f64> for Color {
    fn mul_assign(&mut self, rhs: f64) {
        self.0 *= rhs;
    }
}

impl ops::DivAssign<f64> for Color {
    fn div_assign(&mut self, rhs: f64) {
        self.0 /= rhs;
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ops::Add for Color {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl ops::Sub for Color {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl ops::Mul for Color {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl ops::Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, mut rhs: Color) -> Self::Output {
        rhs *= self;
        rhs
    }
}

impl ops::Mul<f64> for Color {
    type Output = Self;
    fn mul(mut self, rhs: f64) -> Self::Output {
        self *= rhs;
        self
    }
}

impl ops::Div<f64> for Color {
    type Output = Self;

    fn div(mut self, rhs: f64) -> Self::Output {
        self /= rhs;
        self
    }
}
