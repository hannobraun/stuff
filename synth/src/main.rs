use std::{thread::sleep, time::Duration};

use anyhow::anyhow;
use tinyaudio::{run_output_device, OutputDeviceParameters};

fn main() -> anyhow::Result<()> {
    let params = OutputDeviceParameters {
        sample_rate: 48000,
        channels_count: 1,
        channel_sample_count: 48000,
    };

    let mut t = 0.;

    let frequency = 440.;
    let amplitude = 0.1;

    let _device = run_output_device(params, move |data| {
        for value in data {
            t += frequency / params.sample_rate as f32;
            t %= 1.;

            if t < 0.5 {
                *value = 0.;
            } else {
                *value = amplitude;
            }
        }
    })
    .map_err(|err| anyhow!("{}", err))?;

    loop {
        sleep(Duration::from_secs(1));
    }
}