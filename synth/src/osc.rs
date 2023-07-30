use crate::{clock::Clock, signal::Signal};

pub struct Osc {
    pub clock: Clock,
    pub frequency: Box<dyn Signal>,
    pub amplitude: f32,
    pub offset: f32,
    pub wave: fn(f32) -> f32,
}

impl Signal for Osc {
    fn next_value(&mut self) -> f32 {
        self.clock.time +=
            self.frequency.next_value() / self.clock.sample_rate as f32;

        let t = self.clock.time % 1.;
        self.offset + (self.wave)(t) * self.amplitude
    }
}
