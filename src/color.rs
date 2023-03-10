use crate::vec3::Vec3;

pub type Color = Vec3;

pub fn parse(color: Color) -> String{
    let red = (255.999 * color.x()) as u8;
    let green = (255.999 * color.y()) as u8;
    let blue = (255.999 * color.z()) as u8;
    format!("{red} {green} {blue}")
}
