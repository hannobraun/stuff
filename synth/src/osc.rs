pub struct Osc {
    pub clock: f32,
    pub frequency: f32,
    pub amplitude: f32,
}

impl Osc {
    pub fn output(&mut self, sample_rate: f32) -> f32 {
        self.clock += self.frequency / sample_rate;
        self.clock %= 1.;

        if self.clock < 0.5 {
            0.
        } else {
            self.amplitude
        }
    }
}
