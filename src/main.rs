use std::time::Instant;

use hittable::{Hittable, list::HittableList, sphere::Sphere};
use image::{ImageBuffer, Rgb};
use ray::Ray;
use vec3::{Color, Point3, Vec3};

pub mod hittable;
pub mod ray;
pub mod vec3;

fn ray_color(ray: &Ray, world: &dyn Hittable) -> Color {
    if let Some(record) = world.hit(ray, 0.0, f64::INFINITY) {
        return 0.5 * (record.normal() + Color::new([1.0; 3]));
    }
    let unit_direction = ray.direction().unit_vector();
    let a = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - a) * Color::new([1.0; 3]) + a * Color::new([0.5, 0.7, 1.0])
}

fn main() {
    let start = Instant::now();

    // Image dimensions
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 4320;

    // Calculate the image height and ensure the image height is at least 1
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO).max(1.0) as u32;

    // World
    let world = HittableList::new(vec![
        Box::new(Sphere::new(Point3::new([0.0, 0.0, -1.0]), 0.5)),
        Box::new(Sphere::new(Point3::new([0.0, -100.5, -1.0]), 100.0)),
    ]);

    // Camera properties
    // Viewport widths less than one are ok since they are real valued.
    const FOCAL_LENGTH: f64 = 1.0;
    const VIEWPORT_HEIGHT: f64 = 2.0;
    const VIEWPORT_WIDTH: f64 = VIEWPORT_HEIGHT * (IMAGE_WIDTH as f64 / IMAGE_HEIGHT as f64);
    const CAMERA_CENTER: Point3 = Point3::new([0.0, 0.0, 0.0]);

    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    const VIEWPORT_U: Vec3 = Vec3::new([VIEWPORT_WIDTH, 0.0, 0.0]);
    const VIEWPORT_V: Vec3 = Vec3::new([0.0, -VIEWPORT_HEIGHT, 0.0]);

    let pixel_delta_u = VIEWPORT_U / IMAGE_WIDTH as f64;
    let pixel_delta_v = VIEWPORT_V / IMAGE_HEIGHT as f64;

    // Calculate the location of the upper left pixel
    let viewport_upper_left =
        CAMERA_CENTER - Vec3::new([0.0, 0.0, FOCAL_LENGTH]) - VIEWPORT_U / 2.0 - VIEWPORT_V / 2.0;
    let pixel_origin_location = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    ImageBuffer::from_fn(IMAGE_WIDTH, IMAGE_HEIGHT, |x, y| {
        if x == 0 {
            eprint!("\rScanlines remaining: {} ", IMAGE_HEIGHT - y);
        }
        let pixel_center =
            pixel_origin_location + f64::from(x) * pixel_delta_u + f64::from(y) * pixel_delta_v;
        let ray_direction = pixel_center - CAMERA_CENTER;
        let ray = Ray::new(CAMERA_CENTER, ray_direction);
        let pixel_color = ray_color(&ray, &world);
        Rgb::from(pixel_color)
    })
    .save("image.png")
    .unwrap();
    eprintln!("\rDone.                 ");

    println!("{:?}", start.elapsed());
}
