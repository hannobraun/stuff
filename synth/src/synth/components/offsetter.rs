use crate::synth::{
    clock::Clock,
    signal::{IsSignal, Signal},
};

pub struct Offsetter {
    pub input: Signal,
    pub offset: Signal,
}

impl IsSignal for Offsetter {
    fn value(&self, clock: &Clock) -> f32 {
        self.input.value(clock) + self.offset.value(clock)
    }
}
