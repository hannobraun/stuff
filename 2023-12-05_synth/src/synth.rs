use crate::{
    components::{oscillator, Amplify as _},
    signal::{Signal, AMPLIFIER_RANGE, AUDIBLE_RANGE},
    wave,
};

pub fn create() -> Signal {
    let frequency = (220., AUDIBLE_RANGE);
    let volume = (0.1, AMPLIFIER_RANGE);

    oscillator(frequency, wave::sawtooth, AUDIBLE_RANGE).amplify(volume)
}
