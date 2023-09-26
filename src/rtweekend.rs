use rand::distributions;
use std::ops::{Add, Mul, Sub};

#[allow(dead_code)]
pub fn random_range<T: Add<Output = T> + Mul<Output = T> + Sub<Output = T> + Copy>(
    min: T,
    max: T,
) -> T
where
    distributions::Standard: distributions::Distribution<T>,
{
    min + rand::random::<T>() * (max - min)
}
