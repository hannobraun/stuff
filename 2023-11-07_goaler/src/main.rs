use eframe::egui::{
    text::CCursor, text_edit::CCursorRange, CentralPanel, Key, TextEdit,
    TextStyle, Ui,
};

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
            ui.vertical(|ui| {
                ui.label("Foundational Goals");
                ui.horizontal(|ui| {
                    for goal in &mut goals.inner {
                        add_goal(ui, goal);
                    }

                    if ui.button("+").clicked() {
                        goals.add();
                    }
                });
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

fn add_goal(ui: &mut Ui, goal: &mut Goal) {
    let mut output = TextEdit::singleline(&mut goal.name)
        .font(TextStyle::Heading)
        .show(ui);

    if output.response.changed() || output.response.lost_focus() {
        goal.is_new = false;
    }

    if goal.is_new {
        output.state.set_ccursor_range(Some(CCursorRange::two(
            CCursor::new(0),
            CCursor::new(goal.name.len()),
        )));
        output.state.store(ui.ctx(), output.response.id);
        ui.ctx()
            .memory_mut(|memory| memory.request_focus(output.response.id));
    }
}

pub struct Goals {
    inner: Vec<Goal>,
}

impl Goals {
    pub fn add(&mut self) {
        self.inner.push(Goal {
            name: String::from("New Goal"),
            is_new: true,
        });
    }
}

pub struct Goal {
    pub name: String,
    pub is_new: bool,
}
