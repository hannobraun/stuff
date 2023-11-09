mod ui;

fn main() -> anyhow::Result<()> {
    ui::init()?;
    Ok(())
}
