use std::io;

use crate::vec3::Vec3;

pub type Color = Vec3;

pub fn write<W: io::Write>(out: &mut W, color: Color) -> io::Result<()> {
    let red = (255.999 * color.x()) as u8;
    let green = (255.999 * color.y()) as u8;
    let blue = (255.999 * color.z()) as u8;
    writeln!(out, "{red} {green} {blue}")
}
