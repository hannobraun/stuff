use crate::signal::{Audio, Signal, Value};

pub trait Amplify {
    fn amplify(self, factor: impl Into<Signal<Value>>) -> Signal<Audio>;
}

impl Amplify for Signal<Audio> {
    fn amplify(mut self, factor: impl Into<Signal<Value>>) -> Signal<Audio> {
        let mut factor = factor.into();

        Signal::from_fn(move || {
            Audio::new(self.next_value().inner() * factor.next_value().inner())
        })
    }
}
