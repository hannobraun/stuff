#[derive(Clone, Copy)]
pub struct Value(pub f32);

impl From<f32> for Value {
    fn from(value: f32) -> Self {
        Self(value)
    }
}
