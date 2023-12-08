use crate::{
    components::{oscillator, Amplify as _},
    range,
    signal::Signal,
    wave,
};

pub fn create() -> Signal {
    let frequency = (220., range::AUDIBLE);
    let volume = (0.1, range::AMPLIFIER);

    oscillator(frequency, wave::sawtooth, range::AUDIBLE).amplify(volume)
}
