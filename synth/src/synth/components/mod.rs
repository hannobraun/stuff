pub mod offsetter;
pub mod oscillator;
pub mod scaler;

pub trait SynthComponent {
    fn update(&mut self, clock: &super::clock::Clock);
}
