use crate::{components, signal::Signal, wave};

pub fn create<const SAMPLE_RATE: u32>() -> Signal<SAMPLE_RATE> {
    let frequency = 220.;
    let volume = 0.1;

    components::amplify(
        components::oscillator::<SAMPLE_RATE>(wave::sawtooth, frequency),
        volume,
    )
}
