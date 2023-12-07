use crate::signal::range::{AUDIBLE_RANGE, VALUE_RANGE};

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

    pub fn from_frequency(frequency: f32) -> Self {
        assert!(
            AUDIBLE_RANGE.contains(frequency),
            "frequency value must be within human audible range"
        );

        let value = AUDIBLE_RANGE.convert_value_to(frequency, &VALUE_RANGE);

        Self::new(value)
    }

    pub fn inner(&self) -> f32 {
        self.inner
    }

    pub fn as_frequency(&self) -> f32 {
        VALUE_RANGE.convert_value_to(self.inner, &AUDIBLE_RANGE)
    }
}

impl From<f32> for Value {
    fn from(value: f32) -> Self {
        Self::new(value)
    }
}
