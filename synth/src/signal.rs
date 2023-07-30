pub trait Signal {
    fn next_value(&mut self) -> f32;
}
