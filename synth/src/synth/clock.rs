#[derive(Clone, Copy)]
pub struct Clock {
    /// Global time, in fractional seconds
    ///
    /// The denominator of the fraction is defined by the sample rate. So at a
    /// sample rate of 48000, each tick of time would be 1 / 48000 of a second.
    ///
    /// Code using this field assumes that it never overflows. At a sample rate
    /// of 48000, the first overflow should occur after over 12 million years,
    /// according to my calculations, which should work well enough in practice.
    pub time: u64,

    pub sample_rate: u64,
}

impl Clock {
    pub fn advance(&mut self) {
        self.time += 1;
    }

    pub fn t(&self, frequency: f32) -> f32 {
        // We keep counting up `self.time` which, as explained in its
        // documentation, isn't a problem when using `u64`. It can turn into a
        // problem right here though, because we convert it into a floating
        // point number and do math on it. The longer the synth runs, and the
        // larger `time` gets, the less precise the result becomes.
        //
        // I've been testing how this affects the sound quality by setting
        // `time` to a high initial value, thereby simulating a long runtime.
        // Using `f32`, there are clearly audible artifacts at one hour, and it
        // completely breaks down (always results in `0.`) at one day.
        //
        // `f64` is a lot more precise, of course, and to my untrained ear,
        // everything still sounds perfectly fine after 10,000 years. It became
        // sketchy at 100,000 years though, so take that into account before
        // using this code for any non-trivial projects.
        //
        // (I tested with a C3, so maybe give yourself a few orders of magnitude
        // of safety margin, if you need higher frequencies.)
        //
        // That final conversion back into `f32` is fine, as the value will be
        // between 0. and 1. at that point.
        (self.time as f64 / self.sample_rate as f64 * frequency as f64 % 1.)
            as f32
    }
}
