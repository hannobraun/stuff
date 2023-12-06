use super::source::{Fn, SignalSource};

pub struct Signal<T> {
    source: Box<dyn SignalSource<T> + Send>,
}

impl<T> Signal<T> {
    pub fn constant(value: T) -> Self
    where
        T: Copy + Send + 'static,
    {
        Self::from_fn(move || value)
    }

    pub fn from_fn(f: impl FnMut() -> T + Send + 'static) -> Self {
        Self {
            source: Box::new(Fn(f)),
        }
    }

    pub fn next_value(&mut self) -> T {
        self.source.next_value()
    }
}
