use super::Signal;

#[derive(Clone, Copy)]
pub struct Value(f32);

impl Value {
    pub fn new(inner: f32) -> Self {
        Self(inner)
    }

    pub fn inner(&self) -> f32 {
        self.0
    }
}

impl From<f32> for Value {
    fn from(value: f32) -> Self {
        Self::new(value)
    }
}

impl<T> From<T> for Signal<Value>
where
    T: Into<Value>,
{
    fn from(value: T) -> Self {
        let value = value.into();
        Self::constant(value)
    }
}
