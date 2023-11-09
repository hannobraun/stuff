pub struct Goals {
    pub inner: Vec<Goal>,
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
