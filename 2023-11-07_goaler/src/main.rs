fn main() -> anyhow::Result<()> {
    eframe::run_simple_native(
        "Goaler",
        eframe::NativeOptions::default(),
        |_, _| {},
    )
    .map_err(|err| {
        // `eframe::Error` doesn't implement `Send`, so we need to do a
        // conversion here.
        anyhow::anyhow!("{err}")
    })?;
    Ok(())
}
