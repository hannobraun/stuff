#[derive(Clone, Copy)]
pub struct Value {
    value: f32,
}

impl Value {
    pub fn new(value: f32) -> Self {
        assert!(value.is_finite(), "`Value` must not be NaN or infinite");
        assert!(
            (-1. ..=1.).contains(&value),
            "`Value` must be within the range of [-1, 1]"
        );

        Self { value }
    }

    pub fn inner(&self) -> f32 {
        self.value
    }
}
