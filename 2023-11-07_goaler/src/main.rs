use eframe::egui::{self, Key, RichText, Ui};

fn main() -> anyhow::Result<()> {
    let config = eframe::NativeOptions {
        maximized: true,
        ..eframe::NativeOptions::default()
    };

    eframe::run_simple_native("Goaler", config, move |ctx, frame| {
        ctx.input(|input| {
            if input.key_pressed(Key::Escape) {
                frame.close();
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                goal(ui, "Goal 1");
                goal(ui, "Goal 2");
                goal(ui, "Goal 3");
                goal(ui, "Goal 4");
                goal(ui, "Goal 5");
                goal(ui, "Goal 6");
            });
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
    ui.label(RichText::new(name).heading().strong());
}
