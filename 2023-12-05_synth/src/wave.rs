use crate::signal::Audio;

pub type Wave = fn(f32) -> Audio;

pub fn sawtooth(t: f32) -> Audio {
    Audio::new(-1. + t * 2.)
}
