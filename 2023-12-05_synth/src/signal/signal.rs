use super::{
    source::{Fn, SignalSource},
    Value,
};

pub struct Signal {
    source: Box<dyn SignalSource + Send>,
}

impl Signal {
    pub fn from_fn(f: impl FnMut() -> Value + Send + 'static) -> Self {
        Self {
            source: Box::new(Fn(f)),
        }
    }

    pub fn next_value(&mut self) -> Value {
        self.source.next_value()
    }
}

impl From<Value> for Signal {
    fn from(value: Value) -> Self {
        Self::from_fn(move || value)
    }
}
