use std::{cell::Cell, rc::Rc};

use crate::synth::clock::Clock;

#[derive(Default)]
pub struct Output {
    inner: Option<f32>,
}

impl Output {
    pub fn set(&mut self, value: Option<f32>) {
        self.inner = value;
    }

    pub fn get(&self) -> Option<f32> {
        self.inner
    }
}

pub struct Signal {
    inner: Box<dyn HasOutput>,
}

impl Signal {
    pub fn new<T: HasOutput + 'static>(inner: T) -> Self {
        Self {
            inner: Box::new(inner),
        }
    }

    #[allow(unused)]
    pub fn constant(constant: f32) -> Self {
        Self::new(Constant(constant))
    }

    pub fn variable() -> (Self, VariableWriter) {
        let signal = Variable(Rc::new(Cell::new(None)));
        let writer = VariableWriter(signal.0.clone());

        let signal = Self::new(signal);

        (signal, writer)
    }

    pub fn value(&self, clock: &Clock) -> Option<f32> {
        // It might make sense to clamp the value between 0 and 1 here. Then
        // we'd have nice and uniform signals that can be used anywhere. That
        // would require some adjustment on the component side though, as the
        // oscillator interprets its frequency signal in terms of Hz. Either it
        // needs to be made configurable, or we need a separate low-frequency
        // oscillator.
        //
        // Another option is to lean into the fact that this is software, and
        // not actually an electrical signal. Make this fully typed. That's more
        // of a question of which philosophy this project would like to follow.
        self.inner.value(clock)
    }
}

impl Default for Signal {
    fn default() -> Self {
        Self::variable().0
    }
}

pub trait HasOutput {
    fn value(&self, clock: &Clock) -> Option<f32>;
}

pub struct Constant(pub f32);

impl HasOutput for Constant {
    fn value(&self, _: &Clock) -> Option<f32> {
        Some(self.0)
    }
}

pub struct Variable(pub VariableInner);

impl HasOutput for Variable {
    fn value(&self, _: &Clock) -> Option<f32> {
        self.0.get()
    }
}

pub struct VariableWriter(pub VariableInner);

impl VariableWriter {
    pub fn update(&mut self, f: impl FnOnce(Option<f32>) -> Option<f32>) {
        let original = self.0.get();
        let updated = f(original);
        self.0.set(updated);
    }
}

type VariableInner = Rc<Cell<Option<f32>>>;
