#![warn(clippy::nursery, clippy::pedantic)]
#![allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss)]
use std::io::{self, Write};

use color::Color;
use ray::Ray;
use vec3::mul;

use crate::vec3::{Point3, Vec3};

mod vec3;
mod color;
mod ray;

fn hit_sphere(center: Point3, radius: f64, r: &Ray) -> bool{
    let oc = r.origin() - center;
    let a = r.direction().dot(r.direction());
    let b = 2.0 * oc.dot(r.direction());
    let c = radius.mul_add(-radius, oc.dot(oc));
    let discriminant = b.mul_add(b,- 4.0 * a * c);
    discriminant > 0.0
}

fn ray_color(r: &Ray) -> Color{
    if hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, r){
        return Color::new(1.0, 0.0, 0.0);
    }
    let unit_direction = r.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    mul(1.0 - t, Color::new(1.0, 1.0, 1.0)) + mul(t, Color::new(0.5, 0.7, 1.0))
}

fn main() {
    // Image dimensions
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: usize = 400;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal.div(2.0) - vertical.div(2.0) - Vec3::new(0.0, 0.0, focal_length);

    // Render the image
    println!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255");
    for y in (0..IMAGE_HEIGHT).rev(){
        eprint!("\rScanlines remaining: {y}");
        io::stderr().flush().unwrap();
        for x in 0..IMAGE_WIDTH{
            let u = x as f64 / (IMAGE_WIDTH - 1) as f64;
            let v = y as f64 / (IMAGE_HEIGHT - 1) as f64;
            let r = Ray::new(origin, lower_left_corner + mul(u, horizontal) + mul(v,  vertical) - origin);
            let pixel_color = ray_color(&r);
            color::write(pixel_color);
        }
    }
    eprintln!("\nDone");
}
