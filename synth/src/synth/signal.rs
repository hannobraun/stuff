use std::sync::{Arc, RwLock};

use crate::synth::clock::Clock;

pub struct Signal {
    inner: Box<dyn IsSignal + Send>,
}

impl Signal {
    pub fn new<T: IsSignal + Send + 'static>(inner: T) -> Self {
        Self {
            inner: Box::new(inner),
        }
    }

    pub fn constant(constant: f32) -> Self {
        Self::new(Constant(constant))
    }

    pub fn variable(initial: f32) -> (Self, VariableWriter) {
        let signal = Variable(Arc::new(RwLock::new(initial)));
        let writer = VariableWriter(signal.0.clone());

        let signal = Self::new(signal);

        (signal, writer)
    }

    pub fn value(&self, clock: &Clock) -> f32 {
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

pub trait IsSignal {
    fn value(&self, clock: &Clock) -> f32;
}

pub struct Constant(pub f32);

impl IsSignal for Constant {
    fn value(&self, _: &Clock) -> f32 {
        self.0
    }
}

pub struct Variable(pub VariableInner);

impl IsSignal for Variable {
    fn value(&self, _: &Clock) -> f32 {
        *self.0.read().unwrap()
    }
}

pub struct VariableWriter(pub VariableInner);

impl VariableWriter {
    pub fn update(&mut self, f: impl FnOnce(f32) -> f32) {
        let original = *self.0.read().unwrap();
        let updated = f(original);
        *self.0.write().unwrap() = updated;
    }
}

type VariableInner = Arc<RwLock<f32>>;
