use std::panic;

use crossterm::terminal;

use crate::{
    audio::Audio,
    synth::{self},
    ui,
};

pub fn run() -> anyhow::Result<()> {
    terminal::enable_raw_mode()?;
    let result = panic::catch_unwind(run_inner);
    terminal::disable_raw_mode()?;

    // This would probably be a good case for `Result::flatten`, but as of this
    // writing, that is not stable yet.
    match result {
        Ok(Ok(())) => Ok(()),
        Ok(err) => err,
        Err(payload) => panic::resume_unwind(payload),
    }
}

fn run_inner() -> anyhow::Result<()> {
    let audio = Audio::start()?;
    let input = synth::start::start(audio.buffers);
    ui::run(input);

    Ok(())
}
