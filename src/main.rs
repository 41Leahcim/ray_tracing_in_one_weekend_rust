use image::{ImageBuffer, Rgb};
use vec3::Color;

pub mod vec3;

fn main() {
    const IMAGE_WIDTH: u32 = 16382;
    const IMAGE_HEIGHT: u32 = 16382;
    ImageBuffer::from_fn(IMAGE_WIDTH, IMAGE_HEIGHT, |x, y| {
        if x == 0 {
            eprint!("\rScanlines remaining: {} ", IMAGE_HEIGHT - y);
        }
        let pixel_color = Color::new([
            f64::from(x) / f64::from(IMAGE_WIDTH - 1),
            f64::from(y) / f64::from(IMAGE_HEIGHT - 1),
            0.0,
        ]);
        Rgb::from(pixel_color)
    })
    .save("image.ppm")
    .unwrap();
    eprintln!("\rDone.                 ");
}
