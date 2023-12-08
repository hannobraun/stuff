use crate::{
    components::{oscillator, Amplify as _},
    signal::{range, Signal},
    wave,
};

pub fn create() -> Signal {
    let frequency = (220., range::AUDIBLE_RANGE);
    let volume = (0.1, range::AMPLIFIER);

    oscillator(frequency, wave::sawtooth, range::AUDIBLE_RANGE).amplify(volume)
}
