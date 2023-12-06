#[derive(Clone, Copy)]
pub struct Audio {
    value: f32,
}

impl Audio {
    pub fn new(value: f32) -> Self {
        assert!(
            value.is_finite(),
            "`Audio` value must not be NaN or infinite"
        );
        assert!(
            (-1. ..=1.).contains(&value),
            "`Audio` value must be within the range of [-1, 1]"
        );

        Self { value }
    }

    pub fn value(&self) -> f32 {
        self.value
    }
}
