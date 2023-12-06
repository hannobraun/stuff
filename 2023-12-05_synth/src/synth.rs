use crate::{
    components::{amplify, oscillator},
    signal::Signal,
    wave,
};

pub fn create<const SAMPLE_RATE: u32>() -> Signal<SAMPLE_RATE> {
    let frequency = 220.;
    let volume = 0.1;

    amplify(oscillator(wave::sawtooth, frequency), volume)
}
