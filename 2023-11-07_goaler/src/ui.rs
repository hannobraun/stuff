use eframe::egui::{
    text::CCursor, text_edit::CCursorRange, CentralPanel, Key, TextEdit,
    TextStyle, Ui,
};

use crate::model::{GoalView, Goals};

pub fn init() -> anyhow::Result<()> {
    let config = eframe::NativeOptions {
        maximized: true,
        ..eframe::NativeOptions::default()
    };

    let mut goals = Goals::load();
    let mut new_goal = None;

    eframe::run_simple_native("Goaler", config, move |ctx, frame| {
        ctx.input(|input| {
            if input.key_pressed(Key::Escape) {
                frame.close();
            }
        });

        CentralPanel::default().show(ctx, |ui| {
            show_goal_group(ui, "Foundational Goals", |ui| {
                for goal in goals.foundational() {
                    show_goal(ui, goal, &mut new_goal);
                }

                if ui.button("+").clicked() {
                    let id = goals.add_foundational();
                    new_goal = Some(id);
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

fn show_goal_group(ui: &mut Ui, name: &str, f: impl FnOnce(&mut Ui)) {
    ui.group(|ui| {
        ui.label(name);
        f(ui)
    });
}

fn show_goal(ui: &mut Ui, goal: GoalView, new_goal: &mut Option<u64>) {
    ui.group(|ui| {
        ui.vertical(|ui| {
            show_goal_name(ui, goal, new_goal);
            if ui.button("+").clicked() {
                todo!("Can't add sub-goal yet")
            }
        });
    });
}

fn show_goal_name(ui: &mut Ui, mut goal: GoalView, new_goal: &mut Option<u64>) {
    let mut output = TextEdit::singleline(goal.name())
        .font(TextStyle::Heading)
        .show(ui);

    if let Some(id) = *new_goal {
        if id == goal.id() {
            if output.response.changed() || output.response.lost_focus() {
                *new_goal = None;
                return;
            }

            output.state.set_ccursor_range(Some(CCursorRange::two(
                CCursor::new(0),
                CCursor::new(goal.name().len()),
            )));
            output.state.store(ui.ctx(), output.response.id);
            ui.ctx()
                .memory_mut(|memory| memory.request_focus(output.response.id));
        }
    }
}
