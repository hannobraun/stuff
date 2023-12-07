use crate::{
    signal::{Signal, Value, SAMPLE_RATE},
    wave::Wave,
};

pub fn oscillator(
    wave: Wave,
    frequency: impl Into<Signal<Value>>,
) -> Signal<Value> {
    let mut frequency = frequency.into();
    let mut t = 0.;

    Signal::from_fn(move || {
        let value = wave(t);

        t += frequency.next_value().as_frequency() / SAMPLE_RATE as f32;
        t %= 1.;

        value
    })
}
