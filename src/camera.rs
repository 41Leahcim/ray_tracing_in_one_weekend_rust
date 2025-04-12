use std::sync::atomic::{AtomicU32, Ordering};

use image::{ImageBuffer, Rgb};
use rand::random_range;

use crate::{
    hittable::Hittable,
    interval::Interval,
    ray::Ray,
    vec3::{Color, Point3, Vec3},
};

pub struct Camera {
    #[expect(dead_code)]
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

    /// Count of random samples for each pixel
    samples_per_pixel: u16,

    /// Color scale factor for a sum of pixel samples
    pixel_samples_scale: f64,

    /// Maximum number of ray bounces into scene
    max_depth: u8,
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: u32, samples_per_pixel: u16, max_depth: u8) -> Self {
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
            samples_per_pixel,
            pixel_samples_scale: 1.0 / samples_per_pixel as f64,
            max_depth,
        }
    }

    pub fn ray_color(ray: &Ray, depth_left: u8, world: &dyn Hittable) -> Color {
        if depth_left == 0 {
            return Color::default();
        }
        if let Some(record) = world.hit(ray, Interval::new(0.001, f64::INFINITY)) {
            if let Some((attenuation, scattered)) = record.material().scatter(ray, &record) {
                return attenuation * Self::ray_color(&scattered, depth_left - 1, world);
            }
            return Color::default();
        }
        let unit_direction = ray.direction().unit_vector();
        let a = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - a) * Color::new([1.0; 3]) + a * Color::new([0.5, 0.7, 1.0])
    }

    /// Returns the vector to a random point in the [-.5, -.5] - [+5, +5] unit square.
    fn sample_square() -> Vec3 {
        Vec3::new([random_range(-0.5..=0.5), random_range(-0.5..=0.5), 0.0])
    }

    /// Construct a camera ray originating from the origin and directed at randomly sampled points
    /// around the pixel location x, y.
    fn get_ray(&self, x: u32, y: u32) -> Ray {
        let offset = Self::sample_square();
        let pixel_sample = self.pixel_origin_location
            + (f64::from(x) + offset.x()) * self.pixel_delta_u
            + (f64::from(y) + offset.y()) * self.pixel_delta_v;
        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;
        Ray::new(ray_origin, ray_direction)
    }

    pub fn render(&self, world: &(dyn Hittable + Sync)) {
        let pixel_count = AtomicU32::new(0);
        ImageBuffer::from_par_fn(self.image_width, self.image_height, |x, y| {
            let generated_pixels = pixel_count.fetch_add(1, Ordering::Relaxed);
            if generated_pixels % self.image_width == 0 {
                eprint!(
                    "\rScanlines remaining: {} ",
                    self.image_height - (generated_pixels / self.image_width)
                );
            }
            let pixel_color = (0..self.samples_per_pixel)
                .map(|_| {
                    let ray = self.get_ray(x, y);
                    Self::ray_color(&ray, self.max_depth, world)
                })
                .sum::<Color>()
                * self.pixel_samples_scale;
            Rgb::from(pixel_color)
        })
        .save("image.png")
        .unwrap();
        eprintln!("\rDone.                 ");
    }
}
