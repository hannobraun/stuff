use anyhow::anyhow;
use crossbeam_channel::Sender;
use tinyaudio::{
    run_output_device, BaseAudioOutputDevice, OutputDeviceParameters,
};

pub const SAMPLE_RATE: usize = 48_000;
pub const SAMPLE_COUNT: usize = SAMPLE_RATE / 20; // results in 50 ms latency

pub const NUM_CHANNELS: usize = 1;

pub const BUFFER_SIZE: usize = SAMPLE_COUNT * NUM_CHANNELS;
pub type Buffer = [f32; BUFFER_SIZE];

pub struct Audio {
    pub buffers: Sender<Buffer>,
    _device: Box<dyn BaseAudioOutputDevice>,
}

impl Audio {
    pub fn start() -> anyhow::Result<Self> {
        let params = OutputDeviceParameters {
            sample_rate: SAMPLE_RATE,
            channels_count: NUM_CHANNELS,
            channel_sample_count: SAMPLE_COUNT,
        };

        let (tx, rx) = crossbeam_channel::bounded::<Buffer>(0);

        let _device = run_output_device(params, move |data| {
            let new_data = rx.recv().unwrap();
            data.copy_from_slice(&new_data);
        })
        .map_err(|err| anyhow!("{}", err))?;

        Ok(Self {
            buffers: tx,
            _device,
        })
    }
}
