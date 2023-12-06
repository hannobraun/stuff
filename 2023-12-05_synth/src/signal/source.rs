use std::ops::DerefMut;

use super::Value;

pub trait SignalSource {
    fn next_value(&mut self) -> Value;
}

impl<S> SignalSource for Box<S>
where
    S: SignalSource + ?Sized,
{
    fn next_value(&mut self) -> Value {
        self.deref_mut().next_value()
    }
}

pub struct Fn<F>(pub F);

impl<F> SignalSource for Fn<F>
where
    F: FnMut() -> Value,
{
    fn next_value(&mut self) -> Value {
        self.0()
    }
}
