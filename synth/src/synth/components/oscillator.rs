use crate::synth::{
    clock::Clock,
    signal::{HasOutput, Signal},
};

pub struct Oscillator {
    pub frequency: Signal,
    pub wave: fn(f32) -> f32,
}

impl HasOutput for Oscillator {
    fn value(&self, clock: &Clock) -> Option<f32> {
        let frequency = self.frequency.value(clock)?;
        let t = clock.t(frequency);
        Some((self.wave)(t))
    }
}
