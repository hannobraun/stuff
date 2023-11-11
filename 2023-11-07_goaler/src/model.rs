use std::{
    collections::BTreeMap,
    fs::{self, File},
    io::{Read, Write},
};

use serde::{Deserialize, Serialize};
use walkdir::WalkDir;

const GOAL_DIR: &str = "goals";

pub struct Goals {
    inner: BTreeMap<u64, Goal>,
    next_id: u64,
}

impl Goals {
    pub fn load() -> Self {
        let mut inner = BTreeMap::new();
        let mut next_id = 0;

        for entry in WalkDir::new(GOAL_DIR) {
            let entry = entry.unwrap();
            if entry.metadata().unwrap().is_dir() {
                continue;
            }

            let mut toml = String::new();
            File::open(entry.path())
                .unwrap()
                .read_to_string(&mut toml)
                .unwrap();

            let goal: Goal = toml::from_str(&toml).unwrap();

            if goal.id >= next_id {
                next_id = goal.id + 1;
            }

            if inner.insert(goal.id, goal).is_some() {
                panic!("Duplicate ID");
            }
        }

        Self { inner, next_id }
    }

    pub fn foundational(&mut self) -> impl Iterator<Item = GoalView> {
        self.inner.values_mut().map(|goal| GoalView {
            name: goal.name.clone(),
            inner: goal,
        })
    }

    pub fn add_foundational(&mut self) -> u64 {
        let id = self.next_id;
        self.next_id += 1;

        let goal = Goal {
            id,
            name: String::from("New Goal"),
            is_new: true,
        };
        goal.store();

        self.inner.insert(id, goal);

        id
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
