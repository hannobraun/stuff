use crate::signal::Signal;

pub trait Amplify<const SAMPLE_RATE: u32> {
    fn amplify(
        self,
        amplitude: impl Into<Signal<SAMPLE_RATE>>,
    ) -> Signal<SAMPLE_RATE>;
}

impl<const SAMPLE_RATE: u32> Amplify<SAMPLE_RATE> for Signal<SAMPLE_RATE> {
    fn amplify(
        mut self,
        amplitude: impl Into<Signal<SAMPLE_RATE>>,
    ) -> Signal<SAMPLE_RATE> {
        let mut amplitude = amplitude.into();

        Signal::from_fn(move || self.next_value() * amplitude.next_value())
    }
}
