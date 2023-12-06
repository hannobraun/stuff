use super::source::{Fn, SignalSource};

pub struct Signal {
    source: Box<dyn SignalSource + Send>,
}

impl Signal {
    pub fn from_fn(f: impl FnMut() -> f32 + Send + 'static) -> Self {
        Self {
            source: Box::new(Fn(f)),
        }
    }

    pub fn next_value(&mut self) -> f32 {
        self.source.next_value()
    }
}

impl From<f32> for Signal {
    fn from(value: f32) -> Self {
        Self::from_fn(move || value)
    }
}
