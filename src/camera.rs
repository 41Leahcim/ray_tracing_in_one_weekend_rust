use std::io::{self, BufWriter, Write};

use crate::{
    color::{self, Color},
    hittable::{HitRecord, Hittable},
    interval::Interval,
    ray::Ray,
    vec3::{Point3, Vec3},
};

#[derive(Debug, Default)]
pub struct Builder {
    aspect_ratio: Option<f64>,
    image_width: Option<usize>,
    image_height: Option<usize>,
    center: Option<Point3>,
}

#[allow(dead_code)]
impl Builder {
    pub const fn aspect_ratio(mut self, aspect_ratio: f64) -> Self {
        self.aspect_ratio = Some(aspect_ratio);
        self
    }

    pub const fn image_width(mut self, width: usize) -> Self {
        self.image_width = Some(width);
        self
    }

    pub const fn image_height(mut self, height: usize) -> Self {
        self.image_height = Some(height);
        self
    }

    pub const fn center(mut self, center: Point3) -> Self {
        self.center = Some(center);
        self
    }

    /// Returns (`aspect_ratio`, `image_width`, `image_height`)
    fn calculate_resolution(&self) -> (f64, usize, usize) {
        match (self.aspect_ratio, self.image_width, self.image_height) {
            (Some(aspect_ratio), Some(image_width), Some(image_height)) => {
                assert!((image_width as f64 / aspect_ratio - image_height as f64).abs() < 0.001);
                (aspect_ratio, image_width, image_height)
            }
            (Some(aspect_ratio), Some(image_width), None) => {
                let image_height = (image_width as f64 / aspect_ratio).max(1.0) as usize;
                (aspect_ratio, image_width, image_height)
            }
            (Some(aspect_ratio), None, Some(image_height)) => {
                let image_width = (image_height as f64 * aspect_ratio) as usize;
                (aspect_ratio, image_width, image_height)
            }
            (Some(aspect_ratio), None, None) => {
                (aspect_ratio, 100, (100.0 / aspect_ratio) as usize)
            }
            (None, Some(image_width), Some(image_height)) => {
                let aspect_ratio = image_width as f64 / image_height as f64;
                (aspect_ratio, image_width, image_height)
            }
            (None, Some(image_width), None) => (1.0, image_width, image_width),
            (None, None, Some(image_height)) => (1.0, image_height, image_height),
            (None, None, None) => (1.0, 100, 100),
        }
    }

    pub fn buid(self) -> Camera {
        let (_, image_width, image_height) = self.calculate_resolution();
        let center = self.center.unwrap_or_default();

        // Determine viewport dimensions
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        let pixel_delta_u = viewport_u.div(image_width as f64);
        let pixel_delta_v = viewport_v.div(image_height as f64);

        //  Calculate the location of the upper left pixel
        let viewport_upper_left =
            center - Vec3::new(0.0, 0.0, focal_length) - viewport_u.div(2.0) - viewport_v.div(2.0);
        let origin = viewport_upper_left + (pixel_delta_u.mul(0.5) + pixel_delta_v);

        Camera {
            image_width,
            image_height,
            center,
            origin,
            pixel_delta_u,
            pixel_delta_v,
        }
    }
}

pub struct Camera {
    image_width: usize,
    image_height: usize,
    center: Point3,
    origin: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn builder() -> Builder {
        Builder::default()
    }

    pub fn render(&mut self, world: &impl Hittable) {
        let mut out = BufWriter::new(io::stdout().lock());

        writeln!(out, "P3\n{} {}\n255", self.image_width, self.image_height).unwrap();
        for y in 0..self.image_height {
            eprint!("\rScanlines remaining: {}", self.image_height - y);
            for x in 0..self.image_width {
                let pixel_center = self.origin
                    + (self.pixel_delta_u.mul(x as f64))
                    + (self.pixel_delta_v.mul(y as f64));
                let ray_direction = pixel_center - self.center;
                let ray = Ray::new(self.center, ray_direction);
                let pixel_color = Self::ray_color(&ray, world);
                color::write(&mut out, pixel_color)
                    .unwrap_or_else(|e| panic!("Failed to write color value.\n{e}"));
            }
        }
        eprintln!("\nDone");
    }

    fn ray_color(ray: &Ray, world: &impl Hittable) -> Color {
        let mut record = HitRecord::default();

        if world.hit(ray, Interval::new(0.0, f64::INFINITY), &mut record) {
            (record.normal + Color::new(1.0, 1.0, 1.0)).mul(0.5)
        } else {
            let unit_direction = ray.direction().unit_vector();
            let t = 0.5 * (unit_direction.y() + 1.0);
            Color::new(1.0, 1.0, 1.0).mul(1.0 - t) + Color::new(0.5, 0.7, 1.0).mul(t)
        }
    }
}
