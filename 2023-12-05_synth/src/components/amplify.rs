use crate::signal::{Audio, Control, Signal};

pub trait Amplify {
    fn amplify(self, factor: impl Into<Signal<Control>>) -> Signal<Audio>;
}

impl Amplify for Signal<Audio> {
    fn amplify(mut self, factor: impl Into<Signal<Control>>) -> Signal<Audio> {
        let mut factor = factor.into();

        Signal::from_fn(move || {
            Audio::new(self.next_value().value() * factor.next_value().inner())
        })
    }
}
