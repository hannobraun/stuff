use crate::signal::Signal;

pub struct Osc {
    pub clock: f32,
    pub sample_rate: f32,
    pub frequency: Box<dyn Signal>,
    pub amplitude: f32,
    pub offset: f32,
    pub wave: fn(f32) -> f32,
}

impl Signal for Osc {
    fn next_value(&mut self) -> f32 {
        self.clock += self.frequency.next_value() / self.sample_rate;
        self.clock %= 1.;

        self.offset + (self.wave)(self.clock) * self.amplitude
    }
}
