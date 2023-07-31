use crate::synth::{
    clock::Clock,
    signal::{IsSignal, Signal},
};

pub struct Offsetter {
    pub input: Signal,
    pub offset: Signal,
}

impl IsSignal for Offsetter {
    fn value(&self, clock: &Clock) -> Option<f32> {
        self.input
            .value(clock)
            .map(|input| input + self.offset.value(clock).unwrap_or(0.))
    }
}
