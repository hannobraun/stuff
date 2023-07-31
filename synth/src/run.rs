use crate::{
    audio::Audio,
    synth::{self},
    ui,
};

pub fn run() -> anyhow::Result<()> {
    run_inner()
}

fn run_inner() -> anyhow::Result<()> {
    let audio = Audio::start()?;
    let input = synth::start::start(audio.buffers);
    ui::run(input);

    Ok(())
}
