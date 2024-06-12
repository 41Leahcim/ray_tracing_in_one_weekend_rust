use std::time::Instant;

use image::{ImageBuffer, Rgb};

const IMAGE_WIDTH: u32 = 4096;
const IMAGE_HEIGHT: u32 = 4096;

fn main() {
    let start = Instant::now();

    ImageBuffer::from_fn(IMAGE_WIDTH, IMAGE_HEIGHT, |x, y| {
        if x == 0 {
            eprint!("\rScanlines remaining: {}", IMAGE_HEIGHT - y);
        }
        let red = f64::from(x) / f64::from(IMAGE_WIDTH - 1);
        let green = f64::from(y) / f64::from(IMAGE_HEIGHT - 1);
        let blue = 0.0;
        Rgb([
            (red * 255.999) as u8,
            (green * 255.999) as u8,
            (blue * 255.999) as u8,
        ])
    })
    .save("image.png")
    .unwrap();

    eprintln!("\rDone after {} seconds.", start.elapsed().as_secs_f64());
}
