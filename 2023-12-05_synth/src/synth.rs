use synth::{
    components::{oscillator, Amplify as _},
    range,
    signal::Signal,
    wave,
};

pub fn create() -> Signal {
    let frequency = 220.;
    let volume = 0.1;

    oscillator((frequency, range::AUDIBLE), wave::sawtooth, range::AUDIBLE)
        .amplify((volume, range::AMPLIFIER))
}
