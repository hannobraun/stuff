use crate::{
    clock::Clock,
    signal::{IsSignal, Signal},
};

pub struct Osc {
    pub frequency: Signal,
    pub amplitude: f32,
    pub offset: f32,
    pub wave: fn(f32) -> f32,
}

impl IsSignal for Osc {
    fn value(&self, clock: &Clock) -> f32 {
        let t = clock.t(self.frequency.inner.value(clock));
        self.offset + (self.wave)(t) * self.amplitude
    }
}
