use crate::synth::{
    clock::Clock,
    signal::{Input, Output},
};

use super::SynthComponent;

#[derive(Default)]
pub struct Scaler {
    pub input: Input,
    pub scale: Input,
    pub output: Output,
}

impl SynthComponent for Scaler {
    fn update(&mut self, _: &Clock) {
        let scale = self.scale.get().unwrap_or(1.);
        let output = self.input.get().map(|input| input * scale);
        self.output.set(output);
    }
}
