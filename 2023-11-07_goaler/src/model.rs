use std::{
    fs::{self, File},
    io::Write,
};

use serde::{Deserialize, Serialize};

const GOAL_DIR: &str = "goals";

pub struct Goals {
    inner: Vec<Goal>,
    next_id: u64,
}

impl Goals {
    pub fn load() -> Self {
        Self {
            inner: Vec::new(),
            next_id: 0,
        }
    }

    pub fn foundational(&mut self) -> impl Iterator<Item = GoalView> {
        self.inner.iter_mut().map(|goal| GoalView {
            name: goal.name.clone(),
            inner: goal,
        })
    }

    pub fn add_foundational(&mut self) {
        let id = self.next_id;
        self.next_id += 1;

        let goal = Goal {
            id,
            name: String::from("New Goal"),
            is_new: true,
        };
        goal.store();

        self.inner.push(goal);
    }
}

#[derive(Deserialize, Serialize)]
pub struct Goal {
    id: u64,
    name: String,

    /// Uses the default value for `bool`, which is `false`, when deserializing
    #[serde(skip)]
    is_new: bool,
}

impl Goal {
    pub fn store(&self) {
        fs::create_dir_all(GOAL_DIR).unwrap();

        let path = format!("{GOAL_DIR}/{}.toml", self.id);
        let toml = toml::to_string_pretty(self).unwrap();
        File::create(path)
            .unwrap()
            .write_all(toml.as_bytes())
            .unwrap();
    }
}

pub struct GoalView<'r> {
    name: String,
    inner: &'r mut Goal,
}

impl GoalView<'_> {
    pub fn name(&mut self) -> &mut String {
        &mut self.name
    }

    pub fn is_new(&mut self) -> &mut bool {
        &mut self.inner.is_new
    }
}

impl Drop for GoalView<'_> {
    fn drop(&mut self) {
        if self.name != self.inner.name {
            self.inner.name.clone_from(&self.name);
            self.inner.store();
        }
    }
}
