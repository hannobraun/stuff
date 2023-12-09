use crate::signal::Value;

pub trait Wave: Send + 'static {
    fn value_at(&self, t: f32) -> Value;
}

pub struct Sawtooth;

impl Wave for Sawtooth {
    fn value_at(&self, t: f32) -> Value {
        Value::new(-1. + t * 2.)
    }
}

pub struct Square;

impl Wave for Square {
    fn value_at(&self, t: f32) -> Value {
        let value = if t < 0.5 { -1. } else { 1. };
        Value::new(value)
    }
}
