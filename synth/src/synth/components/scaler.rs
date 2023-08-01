use crate::synth::{
    clock::Clock,
    signal::{Output, Signal},
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
        let scale = self.scale.value(clock).unwrap_or(1.);
        let output = self.input.value(clock).map(|input| input * scale);
        self.output.set(output);
    }
}
