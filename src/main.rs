#![warn(clippy::nursery, clippy::pedantic)]
#![allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss)]

use std::io::{self, Write, BufWriter, stdout};
use std::time::Instant;

use color::Color;
use ray::Ray;
use vec3::mul;

use crate::vec3::{Point3, Vec3};

mod vec3;
mod color;
mod ray;

// Image dimensions
const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: usize = 3840;
const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;

fn hit_sphere(center: Point3, radius: f64, r: &Ray) -> f64{
    let oc = r.origin() - center;
    let a = r.direction().dot(r.direction());
    let b = 2.0 * oc.dot(r.direction());
    let c = radius.mul_add(-radius, oc.dot(oc));
    let discriminant = b.mul_add(b,- 4.0 * a * c);
    if discriminant < 0.0{
        -1.0
    }else{
        (-b - discriminant.sqrt()) / (2.0 * a)
    }
}

fn ray_color(r: &Ray) -> Color{
    let t = hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, r);
    if t > 0.0{
        let n = Vec3::unit_vector(r.at(t) - Vec3::new(0.0, 0.0, -1.0));
        Color::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0).mul(0.5)
    }else{
        let unit_direction = r.direction().unit_vector();
        let t = 0.5 * (unit_direction.y() + 1.0);
        Color::new(1.0, 1.0, 1.0).mul(1.0 - t) + Color::new(0.5, 0.7, 1.0).mul(t)
    }
}

fn render_image(origin: Vec3, lower_left_corner: Vec3, horizontal: Vec3, vertical: Vec3){
    let mut out = BufWriter::new(stdout().lock());

    writeln!(out, "P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255").unwrap();
    (0..IMAGE_HEIGHT).rev().for_each(|y|{
        eprint!("\rScanlines remaining: {y}");
        io::stderr().flush().unwrap();
        (0..IMAGE_WIDTH).for_each(|x|{
            let u = x as f64 / (IMAGE_WIDTH - 1) as f64;
            let v = y as f64 / (IMAGE_HEIGHT - 1) as f64;
            let r = Ray::new(origin, lower_left_corner + mul(u, horizontal) + mul(v,  vertical) - origin);
            let pixel_color = ray_color(&r);
            color::write(&mut out, pixel_color).unwrap();
        })
    });
    out.flush().unwrap();
}

fn main() {
    let start = Instant::now();

    // Camera
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal.div(2.0) - vertical.div(2.0) - Vec3::new(0.0, 0.0, focal_length);

    // Render the image
    render_image(origin, lower_left_corner, horizontal, vertical);
    
    eprintln!("\nDone");

    eprintln!("{}", start.elapsed().as_secs_f64());
}
