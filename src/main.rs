#![warn(clippy::pedantic)]

use std::{sync::Arc, time::Instant};

use camera::Camera;
use hittable::{Hittable, list::HittableList, sphere::Sphere};
use material::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal};
use rand::{random, random_range};
use vec3::{Color, Point3, Vec3};

pub mod camera;
pub mod hittable;
pub mod interval;
pub mod material;
pub mod ray;
pub mod vec3;

fn main() {
    let start = Instant::now();

    // World
    let ground_material = Arc::new(Lambertian::new(Color::new([0.5, 0.5, 0.5])));
    let material0 = Arc::new(Dielectric::new(1.5));
    let material1 = Arc::new(Lambertian::new(Color::new([0.4, 0.2, 0.1])));
    let material2 = Arc::new(Metal::new(Color::new([0.7, 0.6, 0.5]), 0.0));

    let objects: [Box<dyn Hittable + Sync>; 4] = [
        Box::new(Sphere::new(
            Point3::new([0.0, -1000.0, 0.0]),
            1000.0,
            ground_material,
        )),
        Box::new(Sphere::new(Point3::new([0.0, 1.0, 0.0]), 1.0, material0)),
        Box::new(Sphere::new(Point3::new([-4.0, 1.0, 0.0]), 1.0, material1)),
        Box::new(Sphere::new(Point3::new([4.0, 1.0, 0.0]), 1.0, material2)),
    ];
    let world = HittableList::new(
        (-11..11)
            .flat_map(|a| (-11..11).map(move |b| (a, b)))
            .filter_map(|(a, b)| {
                let choose_material = random::<f64>();
                let center = Point3::new([
                    f64::from(a) + 0.9 * random::<f64>(),
                    0.2,
                    f64::from(b) + 0.9 * random::<f64>(),
                ]);
                if (center - Point3::new([4.0, 0.2, 0.0])).length() <= 0.9 {
                    return None;
                }
                Some::<Box<dyn Hittable + Sync>>(Box::new(Sphere::new(
                    center,
                    0.2,
                    match choose_material {
                        // Diffuse
                        ..0.8 => {
                            let albedo = Color::random() * Color::random();
                            Arc::new(Lambertian::new(albedo))
                        }
                        ..0.95 => {
                            let albedo = Color::random_range(0.5..1.0);
                            let fuzz = random_range::<f64, _>(0.0..0.5);
                            Arc::new(Metal::new(albedo, fuzz))
                        }
                        _ => Arc::new(Dielectric::new(1.5)),
                    },
                )))
            })
            .chain(objects)
            .collect(),
    );

    let camera = Camera::new(
        16.0 / 9.0,
        1200,
        500,
        50,
        20.0,
        Vec3::new([13.0, 2.0, 3.0]),
        Point3::new([0.0; 3]),
        Vec3::new([0.0, 1.0, 0.0]),
        0.6,
        10.0,
    );
    camera.render(&world);

    println!("{:?}", start.elapsed());
}
