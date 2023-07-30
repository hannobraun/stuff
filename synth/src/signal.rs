use crate::clock::Clock;

pub trait Signal: Send {
    fn value(&mut self, clock: &Clock) -> f32;
}

pub struct Value(pub f32);

impl Signal for Value {
    fn value(&mut self, _: &Clock) -> f32 {
        self.0
    }
}
