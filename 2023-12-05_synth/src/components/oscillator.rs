use crate::{
    signal::{Audio, Frequency, Signal, SAMPLE_RATE},
    wave::Wave,
};

pub fn oscillator(
    wave: Wave,
    frequency: impl Into<Signal<Frequency>>,
) -> Signal<Audio> {
    let mut frequency = frequency.into();
    let mut t = 0.;

    Signal::from_fn(move || {
        let value = wave(t);

        t += frequency.next_value().inner() / SAMPLE_RATE as f32;
        t %= 1.;

        value
    })
}
