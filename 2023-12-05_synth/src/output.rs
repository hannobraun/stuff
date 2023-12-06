use std::ops::DerefMut;

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
            let Some(value) = signal.next_value() else {
                return;
            };

            for sample in samples {
                *sample = value;
            }
        }
    })
    .map_err(|err| anyhow::anyhow!("Audio error: {err:?}"))?;

    Ok(device)
}

struct Signal<const SAMPLE_RATE: u32> {
    inner: Box<dyn SignalSource + Send>,
}

impl<const SAMPLE_RATE: u32> Signal<SAMPLE_RATE> {
    pub fn from_fn(f: impl FnMut() -> f32 + Send + 'static) -> Self {
        Self {
            inner: Box::new(Fn(f)),
        }
    }

    pub fn next_value(&mut self) -> Option<f32> {
        Some(self.inner.next_value())
    }
}

impl<I, const SAMPLE_RATE: u32> From<I> for Signal<SAMPLE_RATE>
where
    I: SignalSource + Send + 'static,
{
    fn from(iter: I) -> Self {
        Self {
            inner: Box::new(iter),
        }
    }
}

pub trait SignalSource {
    fn next_value(&mut self) -> f32;

    fn map<F>(self, f: F) -> Map<Self, F>
    where
        Self: Sized,
    {
        Map { source: self, f }
    }
}

impl<S> SignalSource for Box<S>
where
    S: SignalSource + ?Sized,
{
    fn next_value(&mut self) -> f32 {
        self.deref_mut().next_value()
    }
}

pub struct Fn<F>(F);

impl<F> SignalSource for Fn<F>
where
    F: FnMut() -> f32,
{
    fn next_value(&mut self) -> f32 {
        self.0()
    }
}

pub struct Map<S, F> {
    source: S,
    f: F,
}

impl<S, F> SignalSource for Map<S, F>
where
    S: SignalSource,
    F: FnMut(f32) -> f32,
{
    fn next_value(&mut self) -> f32 {
        (self.f)(self.source.next_value())
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
    signal.inner.map(move |value| value * amplitude).into()
}
