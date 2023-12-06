use super::{
    source::{Fn, SignalSource},
    Value,
};

pub struct Signal {
    source: Box<dyn SignalSource<Value> + Send>,
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

impl<V> From<V> for Signal
where
    V: Into<Value>,
{
    fn from(value: V) -> Self {
        let value = value.into();
        Self::from_fn(move || value)
    }
}
