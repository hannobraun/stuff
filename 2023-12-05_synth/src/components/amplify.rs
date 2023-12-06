use crate::signal::Signal;

pub trait Amplify<const SAMPLE_RATE: u32> {
    fn amplify(self, amplitude: f32) -> Signal<SAMPLE_RATE>;
}

impl<const SAMPLE_RATE: u32> Amplify<SAMPLE_RATE> for Signal<SAMPLE_RATE> {
    fn amplify(self, amplitude: f32) -> Signal<SAMPLE_RATE> {
        self.map(move |value| value * amplitude)
    }
}
