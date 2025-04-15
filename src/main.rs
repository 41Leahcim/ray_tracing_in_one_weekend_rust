use core::f64;
use std::{sync::Arc, time::Instant};

use camera::Camera;
use hittable::{list::HittableList, sphere::Sphere};
use material::lambertian::Lambertian;
use vec3::{Color, Point3};

pub mod camera;
pub mod hittable;
pub mod interval;
pub mod material;
pub mod ray;
pub mod vec3;

fn main() {
    let start = Instant::now();

    // World
    /*let material_ground = Arc::new(Lambertian::new(Color::new([0.8, 0.8, 0.0])));
    let material_center = Arc::new(Lambertian::new(Color::new([0.1, 0.2, 0.5])));
    let material_left = Arc::new(Dielectric::new(1.5));
    let material_bubble = Arc::new(Dielectric::new(1.0 / 1.5));
    let material_right = Arc::new(Metal::new(Color::new([0.8, 0.6, 0.2]), 1.0));

    let world = HittableList::new(vec![
        Box::new(Sphere::new(
            Point3::new([0.0, -100.5, -1.0]),
            100.0,
            material_ground,
        )),
        Box::new(Sphere::new(
            Point3::new([0.0, 0.0, -1.2]),
            0.5,
            material_center,
        )),
        Box::new(Sphere::new(
            Point3::new([-1.0, 0.0, -1.0]),
            0.5,
            material_left,
        )),
        Box::new(Sphere::new(
            Point3::new([-1.0, 0.0, -1.0]),
            0.4,
            material_bubble,
        )),
        Box::new(Sphere::new(
            Point3::new([1.0, 0.0, -1.0]),
            0.5,
            material_right,
        )),
    ]);*/
    let material_left = Arc::new(Lambertian::new(Color::new([0.0, 0.0, 1.0])));
    let material_right = Arc::new(Lambertian::new(Color::new([1.0, 0.0, 0.0])));

    let r = (f64::consts::PI / 4.0).cos();
    let world = HittableList::new(vec![
        Box::new(Sphere::new(Point3::new([-r, 0.0, -1.0]), r, material_left)),
        Box::new(Sphere::new(Point3::new([r, 0.0, -1.0]), r, material_right)),
    ]);

    let camera = Camera::new(16.0 / 9.0, 540, 100, 50, 90.0);
    camera.render(&world);

    println!("{:?}", start.elapsed());
}
