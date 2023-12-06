use crate::signal::Value;

pub type Wave = fn(f32) -> Value;

pub fn sawtooth(t: f32) -> Value {
    -1. + t * 2.
}
