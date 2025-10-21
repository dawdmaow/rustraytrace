use crate::types::Interval;

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    pub fn _size(&self) -> f64 {
        self.max - self.min
    }

    pub fn _contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            self.min
        } else if x > self.max {
            self.max
        } else {
            x
        }
    }

    pub fn empty() -> Self {
        Self::new(f64::INFINITY, f64::NEG_INFINITY)
    }

    pub fn _universe() -> Self {
        Self::new(f64::NEG_INFINITY, f64::INFINITY)
    }
}

impl Default for Interval {
    fn default() -> Self {
        Self::empty()
    }
}
