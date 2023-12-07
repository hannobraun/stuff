pub struct Range {
    pub min: f32,
    pub max: f32,
}

impl Range {
    pub const fn new(min: f32, max: f32) -> Self {
        Self { min, max }
    }

    pub fn width(&self) -> f32 {
        self.max - self.min
    }

    pub fn contains(&self, value: f32) -> bool {
        assert!(value.is_finite());
        (self.min..=self.max).contains(&value)
    }
}

pub const VALUE_RANGE: Range = Range::new(-1., 1.);
pub const AUDIBLE_RANGE: Range = Range::new(20., 20_000.);
