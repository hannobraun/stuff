use eframe::egui::{self, Key, RichText, Ui};

fn main() -> anyhow::Result<()> {
    let config = eframe::NativeOptions {
        maximized: true,
        ..eframe::NativeOptions::default()
    };

    let goals =
        vec!["Goal 1", "Goal 2", "Goal 3", "Goal 4", "Goal 5", "Goal 6"]
            .into_iter()
            .collect::<Vec<_>>();

    eframe::run_simple_native("Goaler", config, move |ctx, frame| {
        ctx.input(|input| {
            if input.key_pressed(Key::Escape) {
                frame.close();
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                for name in &goals {
                    goal(ui, name);
                }
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
