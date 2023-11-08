use eframe::egui::{CentralPanel, Key, TextEdit, TextStyle, Ui};

fn main() -> anyhow::Result<()> {
    let config = eframe::NativeOptions {
        maximized: true,
        ..eframe::NativeOptions::default()
    };

    let mut goals = Goals { inner: Vec::new() };

    eframe::run_simple_native("Goaler", config, move |ctx, frame| {
        ctx.input(|input| {
            if input.key_pressed(Key::Escape) {
                frame.close();
            }
        });

        CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                for goal in &mut goals.inner {
                    add_goal(ui, &mut goal.name);
                }

                if ui.button("+").clicked() {
                    goals.add();
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

fn add_goal(ui: &mut Ui, name: &mut String) {
    ui.add(TextEdit::singleline(name).font(TextStyle::Heading));
}

pub struct Goals {
    inner: Vec<Goal>,
}

impl Goals {
    pub fn add(&mut self) {
        self.inner.push(Goal {
            name: String::from("New Goal"),
        });
    }
}

pub struct Goal {
    pub name: String,
}
