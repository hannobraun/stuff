use crate::signal::range::VALUE_RANGE;

use super::range::Range;

#[derive(Clone, Copy)]
pub struct Value {
    inner: f32,
}

impl Value {
    pub fn new(value: f32) -> Self {
        assert!(
            VALUE_RANGE.contains(value),
            "`Value` must be within the range of [-1, 1]"
        );

        Self { inner: value }
    }

    pub fn encode_from(value: f32, range: &Range) -> Self {
        let value = range.convert_value_to(value, &VALUE_RANGE);
        Self::new(value)
    }

    pub fn inner(&self) -> f32 {
        self.inner
    }

    pub fn decode_to(&self, range: &Range) -> f32 {
        VALUE_RANGE.convert_value_to(self.inner, range)
    }
}

impl From<f32> for Value {
    fn from(value: f32) -> Self {
        Self::new(value)
    }
}
