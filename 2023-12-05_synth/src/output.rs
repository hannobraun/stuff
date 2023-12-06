use std::iter;

pub fn start() -> anyhow::Result<Box<dyn tinyaudio::BaseAudioOutputDevice>> {
    const SAMPLE_RATE: u32 = 48000;

    let params = tinyaudio::OutputDeviceParameters {
        sample_rate: SAMPLE_RATE as usize,
        channels_count: 1,
        channel_sample_count: 4800,
    };

    let frequency = 220.;
    let volume = 0.2;

    let mut oscillator = oscillator::<SAMPLE_RATE>(sawtooth, frequency);

    let device = tinyaudio::run_output_device(params, move |data| {
        for samples in data.chunks_mut(params.channels_count) {
            let Some(value) = oscillator.inner.next() else {
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

struct Signal<const SAMPLE_RATE: u32> {
    inner: Box<dyn Iterator<Item = f32> + Send>,
}

impl<I, const SAMPLE_RATE: u32> From<I> for Signal<SAMPLE_RATE>
where
    I: IntoIterator<Item = f32>,
    I::IntoIter: Send + 'static,
{
    fn from(iter: I) -> Self {
        Self {
            inner: Box::new(iter.into_iter()),
        }
    }
}

fn sawtooth(t: f32) -> f32 {
    -1. + t * 2.
}

fn oscillator<const SAMPLE_RATE: u32>(
    wave: fn(f32) -> f32,
    frequency: f32,
) -> Signal<SAMPLE_RATE> {
    let mut t = 0.;

    iter::from_fn(move || {
        let value = wave(t);

        t += frequency / SAMPLE_RATE as f32;
        t %= 1.;

        Some(value)
    })
    .into()
}
