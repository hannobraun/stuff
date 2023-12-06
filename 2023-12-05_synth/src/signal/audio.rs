#[derive(Clone, Copy)]
pub struct Audio(f32);

impl Audio {
    pub fn new(inner: f32) -> Self {
        assert!(
            inner.is_finite(),
            "`Audio` value must not be NaN or infinite"
        );
        assert!(
            (-1. ..=1.).contains(&inner),
            "`Audio` value must be within the range of [-1, 1]"
        );

        Self(inner)
    }

    pub fn inner(&self) -> f32 {
        self.0
    }
}
