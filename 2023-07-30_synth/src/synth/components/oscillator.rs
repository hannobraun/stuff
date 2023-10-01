use crate::synth::{
    clock::Clock,
    io::{Input, Output},
    wave::Wave,
};

use super::SynthComponent;

#[derive(Default)]
pub struct Oscillator {
    pub frequency: Input,
    pub wave: Wave,
    pub output: Output,
}

impl Oscillator {
    pub fn output(&self, clock: &Clock) -> Option<f32> {
        let frequency = self.frequency.get()?;
        let t = clock.t(frequency);
        Some(self.wave.value(t))
    }
}

impl SynthComponent for Oscillator {
    fn update(&mut self, clock: &Clock) {
        let output = self.output(clock);
        self.output.set(output);
    }
}
