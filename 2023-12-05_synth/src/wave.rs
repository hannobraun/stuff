pub type Wave = fn(f32) -> f32;

pub fn sawtooth(t: f32) -> f32 {
    -1. + t * 2.
}
