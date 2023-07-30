pub trait Signal: Send {
    fn next_value(&mut self) -> f32;
}

pub struct Value(pub f32);

impl Signal for Value {
    fn next_value(&mut self) -> f32 {
        self.0
    }
}
