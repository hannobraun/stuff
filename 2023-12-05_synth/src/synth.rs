use crate::{
    components::{oscillator, Amplify as _},
    signal::Signal,
    wave,
};

pub fn create<const SAMPLE_RATE: u32>() -> Signal<SAMPLE_RATE> {
    let frequency = 220.;
    let volume = 0.1;

    oscillator(wave::sawtooth, frequency).amplify(volume)
}
