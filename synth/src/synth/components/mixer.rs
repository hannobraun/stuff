use crate::synth::{
    clock::Clock,
    io::{Input, Output},
};

use super::SynthComponent;

#[derive(Default)]
pub struct Mixer {
    pub inputs: Vec<Input>,
    pub output: Output,
}

impl Mixer {
    pub fn connect(&mut self, output: &Output) {
        let mut input = Input::default();
        input.connect(output);
        self.inputs.push(input);
    }
}

impl SynthComponent for Mixer {
    fn update(&mut self, _: &Clock) {
        let output = self
            .inputs
            .iter()
            .map(|input| input.get().unwrap_or(0.))
            .sum();
        self.output.set(Some(output));
    }
}
