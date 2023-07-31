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
    ui::{self, UiEvent},
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

    let (note, mut note_writer) = Signal::variable(440.);
    let (volume, mut volume_writer) = Signal::variable(0.1);

    let osc = Signal::new(Oscillator {
        frequency: note,
        wave: wave::square,
    });
    let osc = Signal::new(Scaler {
        input: osc,
        scale: volume,
    });

    let audio = Audio::start()?;
    let ui_events = ui::start();

    let frequency_increment = 20.;
    let volume_increment = 0.1;

    let mut buffer = [0.; BUFFER_SIZE];

    loop {
        let ui_event = select! {
            send(audio.buffers, buffer) -> res => {
                res.unwrap();

                for value in &mut buffer {
                    clock.advance();
                    *value = osc.value(&clock);
                }

                continue;
            }
            recv(ui_events) -> ui_event => {
                ui_event.unwrap()
            }
        };

        match ui_event {
            UiEvent::FrequencyDec => {
                note_writer.update(|freq| freq - frequency_increment);
                continue;
            }
            UiEvent::FrequencyInc => {
                note_writer.update(|freq| freq + frequency_increment);
                continue;
            }

            UiEvent::VolumeDec => {
                volume_writer.update(|volume| volume - volume_increment);
                continue;
            }
            UiEvent::VolumeInc => {
                volume_writer.update(|volume| volume + volume_increment);
                continue;
            }

            UiEvent::Note(note) => {
                let number = match note {
                    ui::Note::C => 4,
                    ui::Note::D => 6,
                    ui::Note::E => 8,
                    ui::Note::F => 9,
                    ui::Note::G => 11,
                    ui::Note::A => 13,
                    ui::Note::B => 15,
                };

                let frequency = 2f32.powf((number - 49) as f32 / 12.) * 440.;

                note_writer.update(|_| frequency);
            }

            UiEvent::Quit => {
                return Ok(());
            }
        }
    }
}
