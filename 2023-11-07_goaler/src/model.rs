pub struct Goals {
    inner: Vec<Goal>,
}

impl Goals {
    pub fn load() -> Self {
        Self { inner: Vec::new() }
    }

    pub fn foundational(&mut self) -> impl Iterator<Item = &mut Goal> {
        self.inner.iter_mut()
    }

    pub fn add_foundational(&mut self) {
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
