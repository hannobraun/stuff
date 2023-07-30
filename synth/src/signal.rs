use crate::clock::Clock;

pub struct Signal {
    inner: Box<dyn IsSignal + Send>,
}

impl Signal {
    pub fn new<T: IsSignal + Send + 'static>(inner: T) -> Self {
        Self {
            inner: Box::new(inner),
        }
    }

    pub fn value(&self, clock: &Clock) -> f32 {
        self.inner.value(clock)
    }
}

pub trait IsSignal {
    fn value(&self, clock: &Clock) -> f32;
}

pub struct Constant(pub f32);

impl IsSignal for Constant {
    fn value(&self, _: &Clock) -> f32 {
        self.0
    }
}
