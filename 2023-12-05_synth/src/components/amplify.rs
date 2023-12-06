use crate::signal::Signal;

pub fn amplify<const SAMPLE_RATE: u32>(
    signal: Signal<SAMPLE_RATE>,
    amplitude: f32,
) -> Signal<SAMPLE_RATE> {
    signal.map(move |value| value * amplitude)
}
