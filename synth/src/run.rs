use std::panic;

use crossbeam_channel::select;
use crossterm::terminal;

use crate::{
    audio::{Audio, BUFFER_SIZE, SAMPLE_RATE},
    synth::{
        clock::Clock,
        components::{oscillator::Oscillator, scaler::Scaler},
        signal::Signal,
        wave,
    },
    ui::{self, Input},
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
    let mut clock = Clock {
        time: 0,
        sample_rate: SAMPLE_RATE as u64,
    };

    let (note, mut note_writer) = Signal::variable();
    let (volume, mut volume_writer) = Signal::variable();
    volume_writer.update(|_| Some(0.1));

    let osc = Signal::new(Oscillator {
        frequency: note,
        wave: wave::sawtooth,
    });
    let osc = Signal::new(Scaler {
        input: osc,
        scale: volume,
    });

    let audio = Audio::start()?;
    let ui_events = ui::start()?;

    let volume_increment = 0.1;

    let mut buffer = [0.; BUFFER_SIZE];
    let mut octave = 0;

    loop {
        let ui_event = select! {
            send(audio.buffers, buffer) -> res => {
                res.unwrap();

                for value in &mut buffer {
                    clock.advance();
                    *value = osc.value(&clock).unwrap_or(0.);
                }

                continue;
            }
            recv(ui_events) -> ui_event => {
                ui_event.unwrap()
            }
        };

        match ui_event {
            Input::OctaveDec => {
                octave -= 1;
                continue;
            }
            Input::OctaveInc => {
                octave += 1;
                continue;
            }

            Input::VolumeDec => {
                volume_writer
                    .update(|volume| Some(volume.unwrap() - volume_increment));
                continue;
            }
            Input::VolumeInc => {
                volume_writer
                    .update(|volume| Some(volume.unwrap() + volume_increment));
                continue;
            }

            Input::PlayNote(note) => {
                let number = match note {
                    ui::Note::C => 4,
                    ui::Note::D => 6,
                    ui::Note::E => 8,
                    ui::Note::F => 9,
                    ui::Note::G => 11,
                    ui::Note::A => 13,
                    ui::Note::B => 15,
                };
                let number = number + octave * 12;

                let frequency = 2f32.powf((number - 49) as f32 / 12.) * 440.;

                note_writer.update(|_| Some(frequency));
            }

            Input::Quit => {
                return Ok(());
            }
        }
    }
}
