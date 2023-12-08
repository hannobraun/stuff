use crate::{
    components::{oscillator, Amplify as _},
    signal::{range, Signal, AUDIBLE_RANGE},
    wave,
};

pub fn create() -> Signal {
    let frequency = (220., AUDIBLE_RANGE);
    let volume = (0.1, range::AMPLIFIER_RANGE);

    oscillator(frequency, wave::sawtooth, AUDIBLE_RANGE).amplify(volume)
}
