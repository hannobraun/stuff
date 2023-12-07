use crate::{
    components::{oscillator, Amplify as _},
    signal::{Signal, Value},
    wave,
};

pub fn create() -> Signal<Value> {
    let frequency = Value::from_frequency(220.);
    let volume = 0.1;

    oscillator(wave::sawtooth, frequency).amplify(volume)
}
