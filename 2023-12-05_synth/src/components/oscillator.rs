use crate::signal::{Signal, SAMPLE_RATE};

pub fn oscillator(
    wave: fn(f32) -> f32,
    frequency: impl Into<Signal>,
) -> Signal {
    let mut frequency = frequency.into();
    let mut t = 0.;

    Signal::from_fn(move || {
        let value = wave(t);

        t += frequency.next_value() / SAMPLE_RATE as f32;
        t %= 1.;

        value
    })
}
