use anyhow::anyhow;
use crossterm::terminal;
use tinyaudio::{run_output_device, OutputDeviceParameters};

use crate::{
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
    let sample_rate = 48000;
    let channel_sample_count = sample_rate / 20; // results in 50 ms latency

    let params = OutputDeviceParameters {
        sample_rate,
        channels_count: 1,
        channel_sample_count,
    };

    let mut clock = Clock {
        time: 0,
        sample_rate: params.sample_rate as u64,
    };

    let (note, mut frequency_writer) = Signal::variable(440.);

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
        scale: Signal::constant(0.1),
    });

    let _device = run_output_device(params, move |data| {
        for value in data {
            clock.advance();
            *value = osc.value(&clock);
        }
    })
    .map_err(|err| anyhow!("{}", err))?;

    let frequency_increment = 20.;

    loop {
        match ui::read_event()? {
            Some(UiEvent::FrequencyDec) => {
                frequency_writer.update(|freq| freq - frequency_increment);
                continue;
            }
            Some(UiEvent::FrequencyInc) => {
                frequency_writer.update(|freq| freq + frequency_increment);
                continue;
            }
            Some(UiEvent::Quit) => {
                return Ok(());
            }
            None => {}
        }
    }
}
