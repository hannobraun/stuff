use crate::synth::{
    clock::Clock,
    signal::{HasOutput, Output, Signal},
};

use super::SynthComponent;

#[derive(Default)]
pub struct Scaler {
    pub input: Signal,
    pub scale: Signal,
    pub output: Output,
}

impl SynthComponent for Scaler {
    fn update(&mut self, clock: &Clock) {
        let output = self
            .input
            .value(clock)
            .map(|input| input * self.scale.value(clock).unwrap_or(1.));
        self.output.set(output);
    }
}

impl HasOutput for Scaler {
    fn value(&self, clock: &Clock) -> Option<f32> {
        self.input
            .value(clock)
            .map(|input| input * self.scale.value(clock).unwrap_or(1.))
    }
}
