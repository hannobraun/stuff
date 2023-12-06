use crate::{
    components::{oscillator, Amplify as _},
    signal::Signal,
    wave,
};

pub fn create() -> Signal {
    let frequency = 220.;
    let volume = 0.1;

    oscillator(wave::sawtooth, frequency).amplify(volume)
}
