use crate::signal::{Signal, Value};

pub trait Amplify {
    fn amplify(self, amplitude: impl Into<Signal>) -> Signal;
}

impl Amplify for Signal {
    fn amplify(mut self, amplitude: impl Into<Signal>) -> Signal {
        let mut amplitude = amplitude.into();

        Signal::from_fn(move || {
            Value(self.next_value().0 * amplitude.next_value().0)
        })
    }
}
