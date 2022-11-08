#![warn(clippy::nursery, clippy::pedantic)]
#![allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss)]
use std::io::{self, Write};

use color::Color;

mod vec3;
mod color;
mod ray;

fn main() {
    // Image dimensions
    const IMAGE_WIDTH: usize = 256;
    const IMAGE_HEIGHT: usize = 256;

    // Render the image
    println!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255");
    for y in (0..IMAGE_HEIGHT).rev(){
        eprint!("\rScanlines remaining: {y}");
        io::stderr().flush().unwrap();
        for x in 0..IMAGE_WIDTH{
            let red = x as f64 / (IMAGE_WIDTH - 1) as f64;
            let green = y as f64 / (IMAGE_HEIGHT - 1) as f64;
            let blue = 0.25;
            let color = Color::new(red, green, blue);
            color::write_color(&color);
        }
    }
    eprintln!("\nDone");
}
