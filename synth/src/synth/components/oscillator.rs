use crate::synth::{
    clock::Clock,
    signal::{HasOutput, Signal},
    wave::Wave,
};

use super::SynthComponent;

#[derive(Default)]
pub struct Oscillator {
    pub frequency: Signal,
    pub wave: Wave,
}

impl SynthComponent for Oscillator {
    fn update(&mut self, _: &Clock) {
        // nothing to do yet
    }
}

impl HasOutput for Oscillator {
    fn value(&self, clock: &Clock) -> Option<f32> {
        let frequency = self.frequency.value(clock)?;
        let t = clock.t(frequency);
        Some(self.wave.value(t))
    }
}
