use crate::{
    clock::Clock,
    signal::{IsSignal, Signal},
};

pub struct Osc {
    pub frequency: Signal,
    pub amplitude: Signal,
    pub offset: Signal,
    pub wave: fn(f32) -> f32,
}

impl IsSignal for Osc {
    fn value(&self, clock: &Clock) -> f32 {
        let t = clock.t(self.frequency.value(clock));
        self.offset.value(clock) + (self.wave)(t) * self.amplitude.value(clock)
    }
}
