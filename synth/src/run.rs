use anyhow::anyhow;
use crossterm::{
    event::{self, Event},
    terminal,
};
use tinyaudio::{run_output_device, OutputDeviceParameters};

use crate::{
    clock::Clock,
    osc::Osc,
    signal::{IsSignal, Signal},
    wave,
};

pub fn run() -> anyhow::Result<()> {
    terminal::enable_raw_mode()?;
    let result = run_inner();
    terminal::disable_raw_mode()?;
    result
}

fn run_inner() -> anyhow::Result<()> {
    let params = OutputDeviceParameters {
        sample_rate: 48000,
        channels_count: 1,
        channel_sample_count: 48000,
    };

    let mut clock = Clock {
        time: 0,
        sample_rate: params.sample_rate as u64,
    };

    let freq_osc = Osc {
        frequency: Signal::constant(1.),
        amplitude: Signal::constant(220.),
        offset: Signal::constant(440.),
        wave: wave::triangle,
    };

    let osc = Osc {
        frequency: Signal::new(freq_osc),
        amplitude: Signal::constant(0.1),
        offset: Signal::constant(0.),
        wave: wave::square,
    };

    let _device = run_output_device(params, move |data| {
        for value in data {
            clock.advance();
            *value = osc.value(&clock);
        }
    })
    .map_err(|err| anyhow!("{}", err))?;

    loop {
        if let Event::Key(key) = event::read()? {
            dbg!(key);
        }
    }
}
