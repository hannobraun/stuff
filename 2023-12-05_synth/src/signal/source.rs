use std::ops::DerefMut;

pub trait SignalSource {
    fn next_value(&mut self) -> f32;
}

impl<S> SignalSource for Box<S>
where
    S: SignalSource + ?Sized,
{
    fn next_value(&mut self) -> f32 {
        self.deref_mut().next_value()
    }
}

pub struct Fn<F>(pub F);

impl<F> SignalSource for Fn<F>
where
    F: FnMut() -> f32,
{
    fn next_value(&mut self) -> f32 {
        self.0()
    }
}
