use std::{
    array,
    fmt::Display,
    iter::Sum,
    ops::{
        Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Range, Sub, SubAssign,
    },
};

use image::Rgb;
use rand::{random, random_range};

#[derive(Debug, Default, Clone, Copy)]
pub struct Vec3([f64; 3]);

impl Vec3 {
    pub const fn new(elements: [f64; 3]) -> Self {
        Self(elements)
    }

    pub const fn x(&self) -> f64 {
        self.0[0]
    }

    pub const fn y(&self) -> f64 {
        self.0[1]
    }

    pub const fn z(&self) -> f64 {
        self.0[2]
    }

    pub fn length_squared(&self) -> f64 {
        self.0.iter().map(|&value| value * value).sum()
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn dot(&self, v: &Self) -> f64 {
        self.0
            .iter()
            .zip(v.0)
            .map(|(left, right)| left * right)
            .sum()
    }

    pub const fn cross(&self, v: &Self) -> Self {
        Self([
            self.0[1] * v.0[2] - self.0[2] * v.0[1],
            self.0[2] * v.0[0] - self.0[0] * v.0[2],
            self.0[0] * v.0[1] - self.0[1] * v.0[0],
        ])
    }

    pub fn unit_vector(self) -> Self {
        self / self.length()
    }

    pub fn random() -> Self {
        Self(array::from_fn(|_| random()))
    }

    pub fn random_range(range: Range<f64>) -> Self {
        Self(array::from_fn(|_| random_range(range.clone())))
    }

    pub fn random_unit_vector() -> Self {
        loop {
            let point = Self::random_range(-1.0..1.0);
            let length_squared = point.length_squared();
            if 1e-160 < length_squared && length_squared <= 1.0 {
                return point / length_squared.sqrt();
            }
        }
    }

    pub fn random_on_hemisphere(&self) -> Self {
        let on_unit_sphere = Self::random_unit_vector();
        if self.dot(&on_unit_sphere) > 0.0 {
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }

    pub fn near_zero(&self) -> bool {
        self.0.iter().all(|&value| value < 1e-8)
    }

    pub fn reflect(self, n: &Self) -> Self {
        self - 2.0 * self.dot(n) * *n
    }

    pub fn refract(&self, n: &Self, etai_over_etat: f64) -> Self {
        let cos_theta = n.dot(&-*self).min(1.0);
        let ray_out_perp = etai_over_etat * (*self + cos_theta * *n);
        let ray_out_parallel = -(1.0 - ray_out_perp.length_squared()).sqrt() * *n;
        ray_out_perp + ray_out_parallel
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        self.0.iter_mut().for_each(|value| *value = -*value);
        self
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.0
            .iter_mut()
            .zip(rhs.0)
            .for_each(|(dest, src)| *dest += src);
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.0
            .iter_mut()
            .zip(rhs.0)
            .for_each(|(dest, src)| *dest -= src);
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self -= rhs;
        self
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        self.0
            .iter_mut()
            .zip(rhs.0)
            .for_each(|(dest, src)| *dest *= src);
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(mut self, rhs: Self) -> Self::Output {
        self *= rhs;
        self
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(mut self, rhs: f64) -> Self::Output {
        self.0.iter_mut().for_each(|value| *value *= rhs);
        self
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, rhs: Self) {
        self.0
            .iter_mut()
            .zip(rhs.0)
            .for_each(|(dest, src)| *dest /= src);
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(mut self, rhs: f64) -> Self::Output {
        self.0.iter_mut().for_each(|value| *value /= rhs);
        self
    }
}

impl Sum for Vec3 {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Color::default(), |result, value| result + value)
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.0[0], self.0[1], self.0[2])
    }
}

// Point3 is just an alias for Vec3, but for geometric clarity in the code.
pub type Point3 = Vec3;

pub type Color = Vec3;

fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        linear_component.sqrt()
    } else {
        0.0
    }
}

impl From<Color> for Rgb<u8> {
    fn from(value: Color) -> Self {
        // Translate the [0, 1) component values to the byte range [0, 255]
        const MAXIMUM: f64 = 0.999;
        let red = (256.0 * linear_to_gamma(value.x()).clamp(0.0, MAXIMUM)) as u8;
        let green = (256.0 * linear_to_gamma(value.y()).clamp(0.0, MAXIMUM)) as u8;
        let blue = (256.0 * linear_to_gamma(value.z()).clamp(0.0, MAXIMUM)) as u8;
        Self([red, green, blue])
    }
}
