use crate::{
    signal::{IsSignal, Signal},
    synth::clock::Clock,
};

pub struct Osc {
    pub frequency: Signal,
    pub amplitude: Signal,
    pub wave: fn(f32) -> f32,
}

impl IsSignal for Osc {
    fn value(&self, clock: &Clock) -> f32 {
        let t = clock.t(self.frequency.value(clock));
        (self.wave)(t) * self.amplitude.value(clock)
    }
}
