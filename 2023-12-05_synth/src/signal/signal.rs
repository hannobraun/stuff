use super::{
    source::{Fn, SignalSource},
    Value,
};

pub struct Signal<T> {
    source: Box<dyn SignalSource<T> + Send>,
}

impl<T> Signal<T> {
    pub fn from_fn(f: impl FnMut() -> T + Send + 'static) -> Self {
        Self {
            source: Box::new(Fn(f)),
        }
    }

    pub fn next_value(&mut self) -> T {
        self.source.next_value()
    }
}

impl<V> From<V> for Signal<Value>
where
    V: Into<Value>,
{
    fn from(value: V) -> Self {
        let value = value.into();
        Self::from_fn(move || value)
    }
}
