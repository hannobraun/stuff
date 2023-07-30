use crate::clock::Clock;

pub struct Signal {
    pub inner: Box<dyn IsSignal>,
}

pub trait IsSignal: Send {
    fn value(&mut self, clock: &Clock) -> f32;
}

pub struct Value(pub f32);

impl IsSignal for Value {
    fn value(&mut self, _: &Clock) -> f32 {
        self.0
    }
}
