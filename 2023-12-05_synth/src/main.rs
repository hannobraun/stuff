mod components;
mod output;
mod signal;
mod wave;

fn main() -> anyhow::Result<()> {
    const SAMPLE_RATE: u32 = 48000;
    let _device = output::start::<SAMPLE_RATE>()?;
    std::thread::sleep(std::time::Duration::from_millis(500));
    Ok(())
}
