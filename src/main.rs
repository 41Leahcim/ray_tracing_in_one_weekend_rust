use std::time::Instant;

use camera::Camera;
use hittable::{list::HittableList, sphere::Sphere};
use vec3::Point3;

pub mod camera;
pub mod hittable;
pub mod interval;
pub mod ray;
pub mod vec3;

fn main() {
    let start = Instant::now();

    // World
    let world = HittableList::new(vec![
        Box::new(Sphere::new(Point3::new([0.0, 0.0, -1.0]), 0.5)),
        Box::new(Sphere::new(Point3::new([0.0, -100.5, -1.0]), 100.0)),
    ]);

    let camera = Camera::new(16.0 / 9.0, 1080, 100);
    camera.render(&world);

    println!("{:?}", start.elapsed());
}
