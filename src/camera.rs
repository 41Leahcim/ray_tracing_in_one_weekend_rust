use image::{ImageBuffer, Rgb};

use crate::{
    hittable::Hittable,
    interval::Interval,
    ray::Ray,
    vec3::{Color, Point3, Vec3},
};

pub struct Camera {
    /// Ratio of image width over height
    aspect_ratio: f64,

    /// Rendered image width in pixel count
    image_width: u32,

    /// Rendered image height in pixels
    image_height: u32,

    /// Camera center
    center: Point3,

    /// Location of pixel 0, 0
    pixel_origin_location: Point3,

    /// Offset to pixel to the right
    pixel_delta_u: Vec3,

    /// Offset to pixel below
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: u32) -> Self {
        // Calculate the image height and ensure the image height is at least 1
        let image_height = (image_width as f64 / aspect_ratio).max(1.0) as u32;

        // Camera properties
        // Viewport widths less than one are ok since they are real valued.
        const FOCAL_LENGTH: f64 = 1.0;
        const VIEWPORT_HEIGHT: f64 = 2.0;
        let viewport_width = VIEWPORT_HEIGHT * (image_width as f64 / image_height as f64);
        const CENTER: Point3 = Point3::new([0.0, 0.0, 0.0]);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u: Vec3 = Vec3::new([viewport_width, 0.0, 0.0]);
        const VIEWPORT_V: Vec3 = Vec3::new([0.0, -VIEWPORT_HEIGHT, 0.0]);

        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = VIEWPORT_V / image_height as f64;

        // Calculate the location of the upper left pixel
        let viewport_upper_left =
            CENTER - Vec3::new([0.0, 0.0, FOCAL_LENGTH]) - viewport_u / 2.0 - VIEWPORT_V / 2.0;
        let pixel_origin_location = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);
        Self {
            aspect_ratio,
            image_width,
            image_height,
            center: CENTER,
            pixel_origin_location,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    pub fn ray_color(ray: &Ray, world: &dyn Hittable) -> Color {
        if let Some(record) = world.hit(ray, Interval::new(0.0, f64::INFINITY)) {
            return 0.5 * (record.normal() + Color::new([1.0; 3]));
        }
        let unit_direction = ray.direction().unit_vector();
        let a = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - a) * Color::new([1.0; 3]) + a * Color::new([0.5, 0.7, 1.0])
    }

    pub fn render(&self, world: &dyn Hittable) {
        ImageBuffer::from_fn(self.image_width, self.image_height, |x, y| {
            if x == 0 {
                eprint!("\rScanlines remaining: {} ", self.image_height - y);
            }
            let pixel_center = self.pixel_origin_location
                + f64::from(x) * self.pixel_delta_u
                + f64::from(y) * self.pixel_delta_v;
            let ray_direction = pixel_center - self.center;
            let ray = Ray::new(self.center, ray_direction);
            let pixel_color = Self::ray_color(&ray, world);
            Rgb::from(pixel_color)
        })
        .save("image.png")
        .unwrap();
        eprintln!("\rDone.                 ");
    }
}
