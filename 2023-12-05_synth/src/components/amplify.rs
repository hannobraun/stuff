use crate::signal::{Control, Signal, Value};

pub trait Amplify {
    fn amplify(self, factor: impl Into<Signal<Control>>) -> Signal<Value>;
}

impl Amplify for Signal<Value> {
    fn amplify(mut self, factor: impl Into<Signal<Control>>) -> Signal<Value> {
        let mut factor = factor.into();

        Signal::from_fn(move || {
            Value::new(self.next_value().inner() * factor.next_value().inner())
        })
    }
}
