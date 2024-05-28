use core::{
    iter,
    ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub},
};

use image::Rgb;

use crate::{interval::Interval, vec3::Vec3};

#[derive(Debug, Clone, Copy)]
pub struct Color(Vec3);

impl Color {
    pub const fn new(red: f64, green: f64, blue: f64) -> Self {
        Self(Vec3::new(red, green, blue))
    }

    pub const fn red(&self) -> f64 {
        self.0.x()
    }

    pub const fn green(&self) -> f64 {
        self.0.y()
    }

    pub const fn blue(&self) -> f64 {
        self.0.z()
    }

    pub fn scale(self, samples_per_pixel: u32) -> Self {
        let scale = 1.0 / f64::from(samples_per_pixel);
        let intensity = Interval::new(0.0, 0.999_999_999_999);
        let red = intensity.clamp(self.0.x() * scale) * 255.0;
        let green = intensity.clamp(self.0.y() * scale) * 255.0;
        let blue = intensity.clamp(self.0.z() * scale) * 255.0;
        Self(Vec3::new(red, green, blue))
    }
}

impl From<Vec3> for Color {
    fn from(value: Vec3) -> Self {
        Self(value)
    }
}

impl From<Color> for Vec3 {
    fn from(value: Color) -> Self {
        value.0
    }
}

impl From<Color> for Rgb<u8> {
    fn from(value: Color) -> Self {
        Self([value.0.x() as u8, value.0.y() as u8, value.0.z() as u8])
    }
}

impl Neg for Color {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl Index<usize> for Color {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        self.0.index(index)
    }
}

impl IndexMut<usize> for Color {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.0.index_mut(index)
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Mul for Color {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Color(rhs.0 * self)
    }
}

impl MulAssign<f64> for Color {
    fn mul_assign(&mut self, rhs: f64) {
        self.0 *= rhs;
    }
}

impl Div<f64> for Color {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self(self.0 / rhs)
    }
}

impl DivAssign<f64> for Color {
    fn div_assign(&mut self, rhs: f64) {
        self.0 /= rhs;
    }
}

impl iter::Sum for Color {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut result = Vec3::default();
        for color in iter {
            result += color.0;
        }
        Self(result)
    }
}
