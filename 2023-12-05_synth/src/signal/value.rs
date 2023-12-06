#[derive(Clone, Copy)]
pub struct Value(pub f32);

impl Value {
    pub fn new(inner: f32) -> Self {
        Self(inner)
    }
}

impl From<f32> for Value {
    fn from(value: f32) -> Self {
        Self::new(value)
    }
}
