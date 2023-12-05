fn main() -> anyhow::Result<()> {
    let params = tinyaudio::OutputDeviceParameters {
        sample_rate: 48000,
        channels_count: 1,
        channel_sample_count: 4800,
    };

    let mut t = 0.;

    let volume = 0.2;

    let _device = tinyaudio::run_output_device(params, move |data| {
        for samples in data.chunks_mut(params.channels_count) {
            let value = -1. + t * 2. * volume;

            for sample in samples {
                *sample = value;
            }

            t += 220. / params.sample_rate as f32;
            t %= 1.;
        }
    })
    .map_err(|err| anyhow::anyhow!("Audio error: {err:?}"))?;

    std::thread::sleep(std::time::Duration::from_secs(1));

    Ok(())
}
