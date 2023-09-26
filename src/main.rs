#![warn(clippy::nursery, clippy::pedantic, clippy::unwrap_used)]
#![allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]

use std::io::{self, stdout, BufWriter, Write};
use std::sync::Arc;
use std::time::Instant;

use color::Color;
use hittable::hittable_list::HittableList;
use hittable::sphere::Sphere;
use hittable::{HitRecord, Hittable};
use ray::Ray;
use vec3::mul;

use crate::vec3::{Point3, Vec3};

mod color;
mod hittable;
mod ray;
mod rtweekend;
mod vec3;

// Image dimensions
const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: usize = 3840;
const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;

fn ray_color(ray: &Ray, world: &impl Hittable) -> Color {
    let mut record = HitRecord::default();

    if world.hit(ray, 0.0, f64::INFINITY, &mut record) {
        (record.normal + Color::new(1.0, 1.0, 1.0)).mul(0.5)
    } else {
        let unit_direction = ray.direction().unit_vector();
        let t = 0.5 * (unit_direction.y() + 1.0);
        Color::new(1.0, 1.0, 1.0).mul(1.0 - t) + Color::new(0.5, 0.7, 1.0).mul(t)
    }
}

fn render_image(
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
) -> io::Result<()> {
    let mut out = BufWriter::new(stdout().lock());
    let mut world = HittableList::default();
    world.add(Arc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Arc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    writeln!(out, "P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255")?;
    for y in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {y}");
        for x in 0..IMAGE_WIDTH {
            let u = x as f64 / (IMAGE_WIDTH - 1) as f64;
            let v = y as f64 / (IMAGE_HEIGHT - 1) as f64;
            let ray = Ray::new(
                origin,
                lower_left_corner + mul(u, horizontal) + mul(v, vertical) - origin,
            );
            let pixel_color = ray_color(&ray, &world);
            color::write(&mut out, pixel_color)?;
        }
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let start = Instant::now();

    // Camera
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal.div(2.0) - vertical.div(2.0) - Vec3::new(0.0, 0.0, focal_length);

    // Render the image
    render_image(origin, lower_left_corner, horizontal, vertical)?;

    eprintln!("\nDone");

    eprintln!("{}", start.elapsed().as_secs_f64());
    Ok(())
}
