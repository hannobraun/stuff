mod components;
mod output;
mod signal;
mod synth;
mod wave;

fn main() -> anyhow::Result<()> {
    let signal = synth::create();
    let _device = output::start(signal)?;

    std::thread::sleep(std::time::Duration::from_millis(200));

    Ok(())
}
