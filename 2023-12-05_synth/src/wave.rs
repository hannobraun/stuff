use crate::signal::Value;

pub type Wave = fn(f32) -> Value;

pub fn sawtooth(t: f32) -> Value {
    Value::new(-1. + t * 2.)
}

pub fn square(t: f32) -> Value {
    let value = if t < 0.5 { -1. } else { 1. };
    Value::new(value)
}
