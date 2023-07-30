pub struct Osc {
    pub clock: f32,
    pub sample_rate: f32,
    pub frequency: f32,
    pub amplitude: f32,
    pub wave: fn(f32) -> f32,
}

impl Osc {
    pub fn next_value(&mut self) -> f32 {
        self.clock += self.frequency / self.sample_rate;
        self.clock %= 1.;

        (self.wave)(self.clock) * self.amplitude
    }
}
