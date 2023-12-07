use crate::signal::{Signal, Value};

pub trait Amplify {
    fn amplify(self, factor: impl Into<Signal<Value>>) -> Signal<Value>;
}

impl Amplify for Signal<Value> {
    fn amplify(mut self, factor: impl Into<Signal<Value>>) -> Signal<Value> {
        let mut factor = factor.into();

        Signal::from_fn(move || {
            Value::new(self.next_value().inner() * factor.next_value().inner())
        })
    }
}
