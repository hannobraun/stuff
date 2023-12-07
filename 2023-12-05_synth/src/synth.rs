use crate::{
    components::{oscillator, Amplify as _},
    signal::{Signal, Value, AUDIBLE_RANGE},
    wave,
};

pub fn create() -> Signal {
    let frequency = Value::encode_from(220., AUDIBLE_RANGE);
    let volume = 0.1;

    oscillator(wave::sawtooth, frequency).amplify(volume)
}
