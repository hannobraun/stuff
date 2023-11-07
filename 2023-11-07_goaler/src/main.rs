fn main() -> anyhow::Result<()> {
    let config = eframe::NativeOptions {
        ..eframe::NativeOptions::default()
    };

    eframe::run_simple_native("Goaler", config, |ctx, frame| {
        frame.set_maximized(true);
        ctx.input(|input| {
            if input.key_pressed(eframe::egui::Key::Escape) {
                frame.close();
            }
        })
    })
    .map_err(|err| {
        // `eframe::Error` doesn't implement `Send`, so we need to do a
        // conversion here.
        anyhow::anyhow!("{err}")
    })?;
    Ok(())
}
