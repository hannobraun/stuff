use std::{cell::Cell, rc::Rc};

#[derive(Default)]
pub struct Input {
    signal: Signal,
}

impl Input {
    pub fn set(&mut self, value: Option<f32>) {
        assert_eq!(
            Rc::strong_count(&self.into()),
            1,
            "Attempting to set connected input."
        );

        self.signal.set(value);
    }

    pub fn get(&self) -> Option<f32> {
        self.signal.get()
    }

    pub fn connect(&mut self, output: &Output) {
        self.signal = output.inner.clone();
    }
}

#[derive(Default)]
pub struct Output {
    inner: Signal,
}

impl Output {
    pub fn set(&mut self, value: Option<f32>) {
        self.inner.set(value);
    }

    pub fn get(&self) -> Option<f32> {
        self.inner.get()
    }
}

type Signal = Rc<Cell<Option<f32>>>;
