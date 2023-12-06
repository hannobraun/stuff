mod components;
mod output;
mod signal;
mod synth;
mod wave;

fn main() -> anyhow::Result<()> {
    const SAMPLE_RATE: u32 = 48000;

    let signal = synth::create();
    let _device = output::start::<SAMPLE_RATE>(signal)?;

    std::thread::sleep(std::time::Duration::from_millis(500));

    Ok(())
}
