use crate::{
    components::{oscillator, Amplify as _},
    signal::{Signal, AUDIBLE_RANGE},
    wave,
};

pub fn create() -> Signal {
    let frequency = (220., AUDIBLE_RANGE);
    let volume = 0.1;

    oscillator(frequency, wave::sawtooth, AUDIBLE_RANGE).amplify(volume)
}
