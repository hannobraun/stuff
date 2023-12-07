use super::Signal;

#[derive(Clone, Copy)]
pub struct Control(f32);

impl Control {
    pub fn new(inner: f32) -> Self {
        Self(inner)
    }
}

impl From<f32> for Control {
    fn from(value: f32) -> Self {
        Self::new(value)
    }
}

impl<T> From<T> for Signal<Control>
where
    T: Into<Control>,
{
    fn from(value: T) -> Self {
        Self::constant(value.into())
    }
}
