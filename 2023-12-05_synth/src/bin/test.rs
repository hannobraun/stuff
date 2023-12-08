use synth::{
    components::{oscillator, Amplify},
    output, range, wave,
};

fn main() -> anyhow::Result<()> {
    let signal = {
        let frequency = 220.;
        let volume = 0.1;

        oscillator((frequency, range::AUDIBLE), wave::sawtooth, range::AUDIBLE)
            .amplify((volume, range::AMPLIFIER))
    };
    let _device = output::start(signal)?;

    std::thread::sleep(std::time::Duration::from_millis(200));

    Ok(())
}
