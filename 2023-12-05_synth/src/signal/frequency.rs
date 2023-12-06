use super::Signal;

#[derive(Clone, Copy)]
pub struct Frequency {
    value: f32,
}

impl Frequency {
    pub fn new(value: f32) -> Self {
        assert!(
            value.is_finite(),
            "`Frequency` value must not be NaN or infinite"
        );
        assert!(
            (20. ..=20_000.).contains(&value),
            "`Frequency` value must be within human audible range"
        );

        Self { value }
    }

    pub fn value(&self) -> f32 {
        self.value
    }
}

impl From<f32> for Frequency {
    fn from(value: f32) -> Self {
        Self::new(value)
    }
}

impl<T> From<T> for Signal<Frequency>
where
    T: Into<Frequency>,
{
    fn from(value: T) -> Self {
        Signal::constant(value.into())
    }
}
