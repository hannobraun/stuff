pub fn square_wave(t: f32) -> f32 {
    if t < 0.5 {
        0.
    } else {
        t
    }
}
