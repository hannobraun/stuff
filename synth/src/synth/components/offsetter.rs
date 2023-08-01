use crate::synth::{
    clock::Clock,
    signal::{HasOutput, Signal},
};

use super::SynthComponent;

pub struct Offsetter {
    pub input: Signal,
    pub offset: Signal,
}

impl SynthComponent for Offsetter {
    fn update(&mut self, _: &Clock) {
        // nothing to do yet
    }
}

impl HasOutput for Offsetter {
    fn value(&self, clock: &Clock) -> Option<f32> {
        self.input
            .value(clock)
            .map(|input| input + self.offset.value(clock).unwrap_or(0.))
    }
}
