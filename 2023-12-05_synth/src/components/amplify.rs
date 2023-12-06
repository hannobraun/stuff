use crate::signal::Signal;

pub trait Amplify {
    fn amplify(self, amplitude: impl Into<Signal>) -> Signal;
}

impl Amplify for Signal {
    fn amplify(mut self, amplitude: impl Into<Signal>) -> Signal {
        let mut amplitude = amplitude.into();

        Signal::from_fn(move || self.next_value() * amplitude.next_value())
    }
}
