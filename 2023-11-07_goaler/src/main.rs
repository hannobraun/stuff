use eframe::egui::{self, Key, Ui};

fn main() -> anyhow::Result<()> {
    let config = eframe::NativeOptions {
        maximized: true,
        ..eframe::NativeOptions::default()
    };

    eframe::run_simple_native("Goaler", config, |ctx, frame| {
        ctx.input(|input| {
            if input.key_pressed(Key::Escape) {
                frame.close();
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            goal(ui, "Goal 1");
        });
    })
    .map_err(|err| {
        // `eframe::Error` doesn't implement `Send`, so we need to do a
        // conversion here.
        anyhow::anyhow!("{err}")
    })?;
    Ok(())
}

fn goal(ui: &mut Ui, name: &str) {
    ui.label(name);
}
