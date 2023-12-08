pub mod components;
pub mod output;
pub mod range;
pub mod signal;
pub mod synth;
pub mod wave;

fn main() -> anyhow::Result<()> {
    let signal = synth::create();
    let _device = output::start(signal)?;

    std::thread::sleep(std::time::Duration::from_millis(200));

    Ok(())
}
