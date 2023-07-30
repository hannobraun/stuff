use std::{thread::sleep, time::Duration};

use anyhow::anyhow;
use tinyaudio::{run_output_device, OutputDeviceParameters};

mod clock;
mod osc;
mod signal;
mod wave;

use self::osc::Osc;

fn main() -> anyhow::Result<()> {
    let params = OutputDeviceParameters {
        sample_rate: 48000,
        channels_count: 1,
        channel_sample_count: 48000,
    };

    let mut clock = clock::Clock {
        time: 0,
        sample_rate: params.sample_rate as u64,
    };

    let freq_osc = Osc {
        frequency: signal::Signal::new(signal::Value(1.)),
        amplitude: 220.,
        offset: 440.,
        wave: wave::triangle,
    };

    let osc = Osc {
        frequency: signal::Signal::new(freq_osc),
        amplitude: 0.1,
        offset: 0.,
        wave: wave::square,
    };

    let _device = run_output_device(params, move |data| {
        for value in data {
            use signal::IsSignal;

            clock.advance();
            *value = osc.value(&clock);
        }
    })
    .map_err(|err| anyhow!("{}", err))?;

    loop {
        sleep(Duration::from_secs(1));
    }
}
