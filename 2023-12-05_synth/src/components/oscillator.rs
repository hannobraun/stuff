use crate::{
    signal::{Signal, SAMPLE_RATE},
    wave::Wave,
};

pub fn oscillator(wave: Wave, frequency: impl Into<Signal>) -> Signal {
    let mut frequency = frequency.into();
    let mut t = 0.;

    Signal::from_fn(move || {
        let value = wave(t);

        t += frequency.next_value() / SAMPLE_RATE as f32;
        t %= 1.;

        value
    })
}
