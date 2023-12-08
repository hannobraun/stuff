use crate::{
    components::{oscillator, Amplify as _},
    signal::{range, range::AUDIBLE_RANGE, Signal},
    wave,
};

pub fn create() -> Signal {
    let frequency = (220., AUDIBLE_RANGE);
    let volume = (0.1, range::AMPLIFIER);

    oscillator(frequency, wave::sawtooth, AUDIBLE_RANGE).amplify(volume)
}
