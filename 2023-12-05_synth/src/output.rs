use crate::signal::Signal;

pub fn start<const SAMPLE_RATE: u32>(
    mut signal: Signal<SAMPLE_RATE>,
) -> anyhow::Result<Box<dyn tinyaudio::BaseAudioOutputDevice>> {
    let sample_rate = SAMPLE_RATE.try_into().expect(
        "Converting `u32` to `usize` should always work on supported platforms",
    );
    let params = tinyaudio::OutputDeviceParameters {
        sample_rate,
        channels_count: 1,
        channel_sample_count: sample_rate / 1000, // results in 1ms latency
    };

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
