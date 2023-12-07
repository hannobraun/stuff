use crate::{
    components::{oscillator, Amplify as _},
    signal::{Signal, Value, AUDIBLE_RANGE},
    wave,
};

pub fn create() -> Signal {
    let frequency = Value::encode_from(220., AUDIBLE_RANGE);
    let volume = 0.1;

    oscillator(frequency, wave::sawtooth, AUDIBLE_RANGE).amplify(volume)
}
