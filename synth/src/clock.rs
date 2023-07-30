#[derive(Clone, Copy)]
pub struct Clock {
    /// Global time, in fractional seconds
    ///
    /// The denominator of the fraction is defined by the sample rate. So at a
    /// sample rate of 48000, each tick of time would be 1 / 48000 of a second.
    ///
    /// Code using this field assumes that it never overflows. At a sample rate
    /// of 48000, the first overflow should occur after more than 12 million
    /// years, according to my calculation, which should work well enough in
    /// practice.
    pub time: u64,

    pub sample_rate: u64,
}

impl Clock {
    pub fn advance(&mut self) {
        self.time += 1;
    }

    pub fn t(&self, frequency: f32) -> f32 {
        // I don't believe this works for timers < 1 Hz. This requires some
        // investigation.
        ((self.time % self.sample_rate) as f64 / self.sample_rate as f64
            * frequency as f64
            % 1.) as f32
    }
}
