#![warn(clippy::nursery, clippy::pedantic, clippy::unwrap_used)]
#![allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]

use std::sync::Arc;
use std::time::Instant;

use hittable::hittable_list::HittableList;
use hittable::sphere::Sphere;

use crate::camera::Camera;
use crate::vec3::Point3;

mod camera;
mod color;
mod hittable;
mod interval;
mod ray;
mod rtweekend;
mod vec3;

// Image dimensions
const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: usize = 480;

fn main() {
    let start = Instant::now();

    let mut world = HittableList::default();
    world.add(Arc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Arc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    let mut cam = Camera::builder()
        .aspect_ratio(ASPECT_RATIO)
        .image_width(IMAGE_WIDTH)
        .samples_per_pixel(100)
        .buid();

    cam.render(&world);

    eprintln!("{}", start.elapsed().as_secs_f64());
}
