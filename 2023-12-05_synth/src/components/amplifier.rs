use crate::signal::Signal;

pub trait Amplify {
    fn amplify(self, factor: impl Into<Signal>) -> Signal;
}

impl Amplify for Signal {
    fn amplify(mut self, factor: impl Into<Signal>) -> Signal {
        let mut factor = factor.into();

        Signal::from_fn(move || self.next_value() * factor.next_value().inner())
    }
}
