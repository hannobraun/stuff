use crate::{
    range::Range,
    signal::{Signal, SAMPLE_RATE},
    wave::Wave,
};

pub fn oscillator(
    frequency: impl Into<Signal>,
    wave: impl Wave,
    range: Range,
) -> Signal {
    let mut frequency = frequency.into();
    let mut t = 0.;

    Signal::from_fn(move || {
        let value = wave.value_at(t);

        t += frequency.next_value().decode_to(range) / SAMPLE_RATE as f32;
        t %= 1.;

        value
    })
}
