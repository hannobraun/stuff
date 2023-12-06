use crate::{
    components::{oscillator, Amplify as _},
    signal::{Signal, SAMPLE_RATE},
    wave,
};

pub fn create() -> Signal<SAMPLE_RATE> {
    let frequency = 220.;
    let volume = 0.1;

    oscillator(wave::sawtooth, frequency).amplify(volume)
}
