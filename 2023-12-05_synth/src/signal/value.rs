#[derive(Clone, Copy)]
pub struct Value {
    inner: f32,
}

impl Value {
    pub fn new(value: f32) -> Self {
        assert!(value.is_finite(), "`Value` must not be NaN or infinite");
        assert!(
            (VALUE_MIN..=VALUE_MAX).contains(&value),
            "`Value` must be within the range of [-1, 1]"
        );

        Self { inner: value }
    }

    pub fn from_frequency(frequency: f32) -> Self {
        assert!(
            (FREQUENCY_MIN..=FREQUENCY_MAX).contains(&frequency),
            "frequency value must be within human audible range"
        );

        let value = VALUE_MIN
            + VALUE_RANGE / FREQUENCY_RANGE * (frequency - FREQUENCY_MIN);

        Self::new(value)
    }

    pub fn inner(&self) -> f32 {
        self.inner
    }

    pub fn as_frequency(&self) -> f32 {
        FREQUENCY_MIN + FREQUENCY_RANGE / VALUE_RANGE * (self.inner - VALUE_MIN)
    }
}

impl From<f32> for Value {
    fn from(value: f32) -> Self {
        Self::new(value)
    }
}

const VALUE_MIN: f32 = -1.0;
const VALUE_MAX: f32 = 1.0;

const FREQUENCY_MIN: f32 = 20.;
const FREQUENCY_MAX: f32 = 20_000.;

const VALUE_RANGE: f32 = VALUE_MAX - VALUE_MIN;
const FREQUENCY_RANGE: f32 = FREQUENCY_MAX - FREQUENCY_MIN;
