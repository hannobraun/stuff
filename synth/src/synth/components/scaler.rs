use crate::synth::{
    clock::Clock,
    signal::{Input, Output, Signal},
};

use super::SynthComponent;

#[derive(Default)]
pub struct Scaler {
    pub input: Signal,
    pub scale: Input,
    pub output: Output,
}

impl SynthComponent for Scaler {
    fn update(&mut self, clock: &Clock) {
        let scale = self.scale.get().unwrap_or(1.);
        let output = self.input.value(clock).map(|input| input * scale);
        self.output.set(output);
    }
}
