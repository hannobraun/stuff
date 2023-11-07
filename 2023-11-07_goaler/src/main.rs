fn main() -> anyhow::Result<()> {
    eframe::run_simple_native(
        "Goaler",
        eframe::NativeOptions::default(),
        |ctx, frame| {
            frame.set_maximized(true);
            ctx.input(|input| {
                if input.key_pressed(eframe::egui::Key::Escape) {
                    frame.close();
                }
            })
        },
    )
    .map_err(|err| {
        // `eframe::Error` doesn't implement `Send`, so we need to do a
        // conversion here.
        anyhow::anyhow!("{err}")
    })?;
    Ok(())
}
