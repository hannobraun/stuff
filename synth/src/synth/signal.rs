use std::{cell::Cell, rc::Rc};

#[derive(Default)]
pub struct Input {
    inner: Rc<Cell<Option<f32>>>,
}

impl Input {
    pub fn set(&mut self, value: Option<f32>) {
        self.inner.set(value);
    }

    pub fn get(&self) -> Option<f32> {
        self.inner.get()
    }

    pub fn connect(&mut self, output: &Output) {
        self.inner = output.inner.clone();
    }
}

#[derive(Default)]
pub struct Output {
    inner: Rc<Cell<Option<f32>>>,
}

impl Output {
    pub fn set(&mut self, value: Option<f32>) {
        self.inner.set(value);
    }

    pub fn get(&self) -> Option<f32> {
        self.inner.get()
    }
}
