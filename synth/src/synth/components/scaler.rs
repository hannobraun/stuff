use crate::synth::{
    clock::Clock,
    signal::{HasOutput, Signal},
};

pub struct Scaler {
    pub input: Signal,
    pub scale: Signal,
}

impl HasOutput for Scaler {
    fn value(&self, clock: &Clock) -> Option<f32> {
        self.input
            .value(clock)
            .map(|input| input * self.scale.value(clock).unwrap_or(1.))
    }
}
