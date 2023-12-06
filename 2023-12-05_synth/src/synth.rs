use crate::{
    components::{oscillator, Amplify as _},
    signal::{Audio, Signal},
    wave,
};

pub fn create() -> Signal<Audio> {
    let frequency = 220.;
    let volume = 0.1;

    oscillator(wave::sawtooth, frequency).amplify(volume)
}
