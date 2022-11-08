use crate::vec3::Color;

pub fn write_color(color: &Color){
    let red = (255.999 * color.x()) as u8;
    let green = (255.999 * color.y()) as u8;
    let blue = (255.999 * color.z()) as u8;
    println!("{red} {green} {blue}"); 
}
