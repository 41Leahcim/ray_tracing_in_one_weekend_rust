#[allow(dead_code)]
pub const EMPTY: Interval = Interval::new(f64::INFINITY, f64::NEG_INFINITY);
#[allow(dead_code)]
pub const UNIVERSE: Interval = Interval::new(f64::NEG_INFINITY, f64::INFINITY);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Interval {
    min: f64,
    max: f64,
}

impl Default for Interval {
    fn default() -> Self {
        Self {
            min: f64::NEG_INFINITY,
            max: f64::INFINITY,
        }
    }
}

impl Interval {
    pub const fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    #[allow(dead_code)]
    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    pub const fn min(&self) -> f64 {
        self.min
    }

    pub const fn max(&self) -> f64 {
        self.max
    }

    pub fn clamp(&self, x: f64) -> f64 {
        x.clamp(self.min, self.max)
    }
}
