use crate::signal::{Signal, Value};

pub trait Amplify {
    fn amplify(self, factor: impl Into<Signal>) -> Signal;
}

impl Amplify for Signal {
    fn amplify(mut self, factor: impl Into<Signal>) -> Signal {
        let mut factor = factor.into();

        Signal::from_fn(move || {
            Value::new(self.next_value().inner() * factor.next_value().inner())
        })
    }
}
