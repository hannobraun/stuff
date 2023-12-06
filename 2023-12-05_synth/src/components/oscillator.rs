use crate::signal::Signal;

pub fn oscillator<const SAMPLE_RATE: u32>(
    wave: fn(f32) -> f32,
    frequency: f32,
) -> Signal<SAMPLE_RATE> {
    let mut t = 0.;

    Signal::from_fn(move || {
        let value = wave(t);

        t += frequency / SAMPLE_RATE as f32;
        t %= 1.;

        value
    })
}
