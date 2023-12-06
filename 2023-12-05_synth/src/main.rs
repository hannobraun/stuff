mod output;
mod signal;
mod wave;

fn main() -> anyhow::Result<()> {
    let _device = output::start()?;
    std::thread::sleep(std::time::Duration::from_millis(500));
    Ok(())
}
