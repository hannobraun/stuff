use super::{
    source::{Fn, SignalSource},
    Value,
};

pub struct Signal {
    source: Box<dyn SignalSource<Value> + Send>,
}

impl Signal {
    pub fn constant(value: Value) -> Self {
        Self::from_fn(move || value)
    }

    pub fn from_fn(f: impl FnMut() -> Value + Send + 'static) -> Self {
        Self {
            source: Box::new(Fn(f)),
        }
    }

    pub fn next_value(&mut self) -> Value {
        self.source.next_value()
    }
}
