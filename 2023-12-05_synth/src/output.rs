use std::iter;

pub fn start() -> anyhow::Result<Box<dyn tinyaudio::BaseAudioOutputDevice>> {
    let params = tinyaudio::OutputDeviceParameters {
        sample_rate: 48000,
        channels_count: 1,
        channel_sample_count: 4800,
    };

    let frequency = 220.;
    let volume = 0.2;

    let mut oscillator =
        oscillator(sawtooth, frequency, params.sample_rate as f32);

    let device = tinyaudio::run_output_device(params, move |data| {
        for samples in data.chunks_mut(params.channels_count) {
            let Some(value) = oscillator.next() else {
                return;
            };
            let value = value * volume;

            for sample in samples {
                *sample = value;
            }
        }
    })
    .map_err(|err| anyhow::anyhow!("Audio error: {err:?}"))?;

    Ok(device)
}

fn sawtooth(t: f32) -> f32 {
    -1. + t * 2.
}

fn oscillator(
    wave: fn(f32) -> f32,
    frequency: f32,
    sample_rate: f32,
) -> impl Iterator<Item = f32> {
    let mut t = 0.;

    iter::from_fn(move || {
        let value = wave(t);

        t += frequency / sample_rate;
        t %= 1.;

        Some(value)
    })
}
