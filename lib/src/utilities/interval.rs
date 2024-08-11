pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    // https://stackoverflow.com/questions/26549480/how-do-i-declare-a-static-field-in-a-struct-in-rust
    const EMPTY: Interval = Interval::new(std::f64::INFINITY, std::f64::NEG_INFINITY);
    const UNIVERSE: Interval = Interval::new(std::f64::NEG_INFINITY, std::f64::INFINITY);

    pub const fn new(min: f64, max: f64) -> Self {
        // Needs to add it as const fn, since EMPTY const needs a const function
        Self { min, max }
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn contains(&self, element: f64) -> bool {
        element >= self.min && element <= self.max
    }

    pub fn inside(&self, element: f64) -> bool {
        element > self.min && element < self.max
    }
}

impl Default for Interval {
    fn default() -> Self {
        Self {
            min: std::f64::INFINITY,
            max: std::f64::NEG_INFINITY,
        }
    }
}
