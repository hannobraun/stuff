use std::{thread::sleep, time::Duration};

use anyhow::anyhow;
use tinyaudio::{run_output_device, OutputDeviceParameters};

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

    let mut osc = Osc {
        clock: 0.,
        sample_rate: params.sample_rate as f32,
        frequency: Box::new(signal::Value(440.)),
        amplitude: 0.1,
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
