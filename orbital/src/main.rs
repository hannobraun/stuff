mod event_loop;

fn main() -> anyhow::Result<()> {
    event_loop::run()?;
    Ok(())
}
