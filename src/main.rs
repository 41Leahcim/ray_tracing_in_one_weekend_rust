#![warn(clippy::nursery, clippy::pedantic, clippy::restriction)]
#![allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::missing_docs_in_private_items,
    clippy::blanket_clippy_restriction_lints,
    clippy::print_stderr,
    clippy::implicit_return,
    clippy::arithmetic_side_effects,
    clippy::default_numeric_fallback,
    clippy::float_arithmetic,
    clippy::as_conversions,
    clippy::single_call_fn,
    clippy::panic,
    clippy::self_named_module_files,
    clippy::expect_used
)]

extern crate alloc;

use alloc::sync::Arc;
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
const IMAGE_WIDTH: usize = 1920;

fn main() {
    let start = Instant::now();

    let mut world = HittableList::default();
    world.add(Arc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Arc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    let cam = Camera::builder()
        .aspect_ratio(ASPECT_RATIO)
        .image_width(IMAGE_WIDTH)
        .samples_per_pixel(100)
        .buid();

    cam.render(&world);

    eprintln!("{}", start.elapsed().as_secs_f64());
}
