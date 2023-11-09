use serde::{Deserialize, Serialize};

pub struct Goals {
    inner: Vec<Goal>,
}

impl Goals {
    pub fn load() -> Self {
        Self { inner: Vec::new() }
    }

    pub fn foundational(&mut self) -> impl Iterator<Item = GoalView> {
        self.inner.iter_mut().map(|goal| GoalView {
            name: goal.name.clone(),
            inner: goal,
        })
    }

    pub fn add_foundational(&mut self) {
        self.inner.push(Goal {
            name: String::from("New Goal"),
            is_new: true,
        });
    }
}

#[derive(Deserialize, Serialize)]
pub struct Goal {
    name: String,

    /// Uses the default value for `bool`, which is `false`, when deserializing
    #[serde(skip)]
    is_new: bool,
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
        }
    }
}
