use crate::{clock::Clock, signal::IsSignal};

pub struct Osc {
    pub frequency: Box<dyn IsSignal>,
    pub amplitude: f32,
    pub offset: f32,
    pub wave: fn(f32) -> f32,
}

impl IsSignal for Osc {
    fn value(&mut self, clock: &Clock) -> f32 {
        let t = clock.t(self.frequency.value(clock));
        self.offset + (self.wave)(t) * self.amplitude
    }
}
