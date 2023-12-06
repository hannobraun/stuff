use crate::{signal::Signal, wave::sawtooth};

pub fn start() -> anyhow::Result<Box<dyn tinyaudio::BaseAudioOutputDevice>> {
    const SAMPLE_RATE: u32 = 48000;

    let sample_rate = SAMPLE_RATE.try_into().expect(
        "Converting `u32` to `usize` should always work on supported platforms",
    );
    let params = tinyaudio::OutputDeviceParameters {
        sample_rate,
        channels_count: 1,
        channel_sample_count: sample_rate / 1000, // results in 1ms latency
    };

    let frequency = 220.;
    let volume = 0.1;

    let mut signal =
        amplify(oscillator::<SAMPLE_RATE>(sawtooth, frequency), volume);

    let device = tinyaudio::run_output_device(params, move |data| {
        for samples in data.chunks_mut(params.channels_count) {
            let value = signal.next_value();

            for sample in samples {
                *sample = value;
            }
        }
    })
    .map_err(|err| anyhow::anyhow!("Audio error: {err:?}"))?;

    Ok(device)
}

fn oscillator<const SAMPLE_RATE: u32>(
    wave: fn(f32) -> f32,
    frequency: f32,
) -> Signal<SAMPLE_RATE> {
    let mut t = 0.;

    Signal::from_fn(move || {
        let value = wave(t);

        t += frequency / SAMPLE_RATE as f32;
        t %= 1.;

        value
    })
}

fn amplify<const SAMPLE_RATE: u32>(
    signal: Signal<SAMPLE_RATE>,
    amplitude: f32,
) -> Signal<SAMPLE_RATE> {
    signal.map(move |value| value * amplitude)
}
