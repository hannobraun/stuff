use crate::{
    audio::Audio,
    synth::{self},
    ui,
};

pub fn run() -> anyhow::Result<()> {
    let audio = Audio::start()?;
    let input = synth::start::start(audio.buffers);
    ui::run(input)?;

    Ok(())
}
