use std::io;

use crate::{interval::Interval, vec3::Vec3};

pub type Color = Vec3;

pub fn write<W: io::Write>(out: &mut W, color: Color, samples_per_pixel: u32) -> io::Result<()> {
    let red = color.x();
    let green = color.y();
    let blue = color.z();

    #[allow(clippy::cast_lossless)]
    let scale = 1.0 / samples_per_pixel as f64;
    let intensity = Interval::new(0.0, 0.999_999_999_999);
    let red = (intensity.clamp(red * scale) * 256.0) as u8;
    let green = (intensity.clamp(green * scale) * 256.0) as u8;
    let blue = (intensity.clamp(blue * scale) * 256.0) as u8;
    writeln!(out, "{red} {green} {blue}")
}
