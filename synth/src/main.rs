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

    let clock = clock::Clock {
        time: 0.,
        sample_rate: params.sample_rate as f32,
    };

    let freq_osc = Osc {
        clock,
        frequency: Box::new(signal::Value(1.)),
        amplitude: 220.,
        offset: 440.,
        wave: wave::triangle,
    };

    let mut osc = Osc {
        clock,
        frequency: Box::new(freq_osc),
        amplitude: 0.1,
        offset: 0.,
        wave: wave::square,
    };

    let _device = run_output_device(params, move |data| {
        for value in data {
            use signal::Signal;
            *value = osc.next_value();
        }
    })
    .map_err(|err| anyhow!("{}", err))?;

    loop {
        sleep(Duration::from_secs(1));
    }
}
