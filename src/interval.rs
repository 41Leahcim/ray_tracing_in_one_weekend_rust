#[derive(Debug, Clone, Copy)]
pub struct Interval {
    min: f64,
    max: f64,
}

impl Default for Interval {
    fn default() -> Self {
        Self {
            min: f64::INFINITY,
            max: f64::NEG_INFINITY,
        }
    }
}

impl Interval {
    #[must_use]
    pub const fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    #[must_use]
    pub const fn size(&self) -> f64 {
        self.max - self.min
    }

    #[must_use]
    pub const fn contains(&self, value: f64) -> bool {
        self.min <= value && value <= self.max
    }

    #[must_use]
    pub const fn surrounds(&self, value: f64) -> bool {
        self.min < value && value <= self.max
    }

    #[must_use]
    pub const fn min(&self) -> f64 {
        self.min
    }

    #[must_use]
    pub const fn max(&self) -> f64 {
        self.max
    }

    pub const EMPTY: Self = Self::new(f64::INFINITY, f64::NEG_INFINITY);
    pub const UNIVERSE: Self = Self::new(f64::NEG_INFINITY, f64::INFINITY);
}
