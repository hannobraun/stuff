pub mod synth;

fn main() -> anyhow::Result<()> {
    let signal = synth::create();
    let _device = ::synth::output::start(signal)?;

    std::thread::sleep(std::time::Duration::from_millis(200));

    Ok(())
}
