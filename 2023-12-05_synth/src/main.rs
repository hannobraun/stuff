mod components;
mod output;
mod signal;
mod wave;

fn main() -> anyhow::Result<()> {
    const SAMPLE_RATE: u32 = 48000;

    let frequency = 220.;
    let volume = 0.1;

    let signal = components::amplify(
        components::oscillator::<SAMPLE_RATE>(wave::sawtooth, frequency),
        volume,
    );

    let _device = output::start::<SAMPLE_RATE>(signal)?;

    std::thread::sleep(std::time::Duration::from_millis(500));

    Ok(())
}
