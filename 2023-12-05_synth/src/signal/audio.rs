#[derive(Clone, Copy)]
pub struct Audio(f32);

impl Audio {
    pub fn new(inner: f32) -> Self {
        Self(inner)
    }

    pub fn inner(&self) -> f32 {
        self.0
    }
}
