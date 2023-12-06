use super::source::{Fn, SignalSource};

pub struct Signal<const SAMPLE_RATE: u32> {
    source: Box<dyn SignalSource + Send>,
}

impl<const SAMPLE_RATE: u32> Signal<SAMPLE_RATE> {
    pub fn from_fn(f: impl FnMut() -> f32 + Send + 'static) -> Self {
        Self {
            source: Box::new(Fn(f)),
        }
    }

    pub fn next_value(&mut self) -> f32 {
        self.source.next_value()
    }
}

impl<const SAMPLE_RATE: u32> From<f32> for Signal<SAMPLE_RATE> {
    fn from(value: f32) -> Self {
        Self::from_fn(move || value)
    }
}
