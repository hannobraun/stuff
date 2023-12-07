use crate::{
    components::{oscillator, Amplify as _},
    signal::{Signal, Value, AUDIBLE_RANGE},
    wave,
};

pub fn create() -> Signal {
    let frequency = Value::from_frequency(220., &AUDIBLE_RANGE);
    let volume = 0.1;

    oscillator(wave::sawtooth, frequency).amplify(volume)
}
