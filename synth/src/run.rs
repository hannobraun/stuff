use anyhow::anyhow;
use crossterm::terminal;
use tinyaudio::{run_output_device, OutputDeviceParameters};

use crate::{
    audio::{SAMPLE_COUNT, SAMPLE_RATE},
    synth::{
        clock::Clock,
        components::{
            offsetter::Offsetter, oscillator::Oscillator, scaler::Scaler,
        },
        signal::Signal,
        wave,
    },
    ui::{self, UiEvent},
};

pub fn run() -> anyhow::Result<()> {
    terminal::enable_raw_mode()?;
    let result = run_inner();
    terminal::disable_raw_mode()?;
    result
}

fn run_inner() -> anyhow::Result<()> {
    let params = OutputDeviceParameters {
        sample_rate: SAMPLE_RATE,
        channels_count: 1,
        channel_sample_count: SAMPLE_COUNT,
    };

    let mut clock = Clock {
        time: 0,
        sample_rate: params.sample_rate as u64,
    };

    let (note, mut note_writer) = Signal::variable(440.);
    let (volume, mut volume_writer) = Signal::variable(0.1);

    let frequency = Signal::new(Oscillator {
        frequency: Signal::constant(1.),
        wave: wave::triangle,
    });
    let frequency = Signal::new(Scaler {
        input: frequency,
        scale: Signal::constant(220.),
    });
    let frequency = Signal::new(Offsetter {
        input: frequency,
        offset: note,
    });

    let osc = Signal::new(Oscillator {
        frequency,
        wave: wave::square,
    });
    let osc = Signal::new(Scaler {
        input: osc,
        scale: volume,
    });

    let _device = run_output_device(params, move |data| {
        for value in data {
            clock.advance();
            *value = osc.value(&clock);
        }
    })
    .map_err(|err| anyhow!("{}", err))?;

    let frequency_increment = 20.;
    let volume_increment = 0.1;

    loop {
        match ui::read_event()? {
            Some(UiEvent::FrequencyDec) => {
                note_writer.update(|freq| freq - frequency_increment);
                continue;
            }
            Some(UiEvent::FrequencyInc) => {
                note_writer.update(|freq| freq + frequency_increment);
                continue;
            }

            Some(UiEvent::VolumeDec) => {
                volume_writer.update(|volume| volume - volume_increment);
                continue;
            }
            Some(UiEvent::VolumeInc) => {
                volume_writer.update(|volume| volume + volume_increment);
                continue;
            }

            Some(UiEvent::Quit) => {
                return Ok(());
            }
            None => {}
        }
    }
}
