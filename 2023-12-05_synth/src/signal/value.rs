use std::ops;

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

    pub fn encode_from(value: f32, range: Range) -> Self {
        let value = range.convert_value_to(value, VALUE_RANGE);
        Self::new(value)
    }

    pub fn inner(&self) -> f32 {
        self.inner
    }

    pub fn decode_to(&self, range: Range) -> f32 {
        VALUE_RANGE.convert_value_to(self.inner, range)
    }
}

impl ops::Mul<f32> for Value {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        let result = (self.inner * rhs).clamp(VALUE_RANGE.min, VALUE_RANGE.max);
        Self::new(result)
    }
}

impl From<f32> for Value {
    fn from(value: f32) -> Self {
        Self::new(value)
    }
}

impl From<(f32, Range)> for Value {
    fn from((value, range): (f32, Range)) -> Self {
        Self::encode_from(value, range)
    }
}
