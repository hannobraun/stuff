use std::{thread::sleep, time::Duration};

use anyhow::anyhow;
use tinyaudio::{run_output_device, OutputDeviceParameters};

mod osc;

use self::osc::Osc;

fn main() -> anyhow::Result<()> {
    let params = OutputDeviceParameters {
        sample_rate: 48000,
        channels_count: 1,
        channel_sample_count: 48000,
    };

    let mut osc = Osc {
        clock: 0.,
        frequency: 440.,
        amplitude: 0.1,
    };

    let _device = run_output_device(params, move |data| {
        for value in data {
            osc.clock += osc.frequency / params.sample_rate as f32;
            osc.clock %= 1.;

            if osc.clock < 0.5 {
                *value = 0.;
            } else {
                *value = osc.amplitude;
            }
        }
    })
    .map_err(|err| anyhow!("{}", err))?;

    loop {
        sleep(Duration::from_secs(1));
    }
}
