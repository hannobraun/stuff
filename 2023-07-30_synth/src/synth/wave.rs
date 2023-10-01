pub struct Wave {
    inner: fn(f32) -> f32,
}

impl Wave {
    pub fn sawtooth() -> Self {
        Self { inner: sawtooth }
    }

    pub fn value(&self, t: f32) -> f32 {
        (self.inner)(t)
    }
}

impl Default for Wave {
    fn default() -> Self {
        Self::sawtooth()
    }
}

fn sawtooth(t: f32) -> f32 {
    -2. * t + 1.
}
