use crate::synth::{
    clock::Clock,
    signal::{IsSignal, Signal},
};

pub struct Scaler {
    pub input: Signal,
    pub scale: Signal,
}

impl IsSignal for Scaler {
    fn value(&self, clock: &Clock) -> f32 {
        self.input.value(clock) * self.scale.value(clock)
    }
}
