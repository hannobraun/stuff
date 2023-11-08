use eframe::egui::{CentralPanel, Key, TextEdit, TextStyle, Ui};

fn main() -> anyhow::Result<()> {
    let config = eframe::NativeOptions {
        maximized: true,
        ..eframe::NativeOptions::default()
    };

    let mut goals = Vec::new();

    eframe::run_simple_native("Goaler", config, move |ctx, frame| {
        ctx.input(|input| {
            if input.key_pressed(Key::Escape) {
                frame.close();
            }
        });

        CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                for name in &mut goals {
                    goal(ui, name);
                }

                if ui.button("+").clicked() {
                    goals.push(String::from("New Goal"));
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

fn goal(ui: &mut Ui, name: &mut String) {
    ui.add(TextEdit::singleline(name).font(TextStyle::Heading));
}
