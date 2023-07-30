pub trait Signal: Send {
    fn next_value(&mut self) -> f32;
}
