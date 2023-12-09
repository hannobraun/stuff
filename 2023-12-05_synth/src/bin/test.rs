use synth::{
    components::{oscillator, Amplify, Limit},
    output, range, wave,
};

fn main() -> anyhow::Result<()> {
    let signal = {
        let frequency = 220.;
        let volume = 0.1;

        let gain = oscillator((20., range::LFO), wave::Square, range::LFO)
            .limit(
                (0., range::AMPLIFIER),
                (volume, range::AMPLIFIER),
                range::AMPLIFIER,
            );

        oscillator((frequency, range::AUDIBLE), wave::Sawtooth, range::AUDIBLE)
            .amplify(gain)
    };
    let _device = output::start(signal)?;

    std::thread::sleep(std::time::Duration::from_millis(200));

    Ok(())
}
