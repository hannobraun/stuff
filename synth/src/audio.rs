use crossbeam_channel::Sender;
use tinyaudio::BaseAudioOutputDevice;

pub const SAMPLE_RATE: usize = 48_000;
pub const SAMPLE_COUNT: usize = SAMPLE_RATE / 20; // results in 50 ms latency

pub const NUM_CHANNELS: usize = 1;

pub const BUFFER_SIZE: usize = SAMPLE_COUNT * NUM_CHANNELS;
pub type Buffer = [f32; BUFFER_SIZE];

pub struct Audio {
    pub buffers: Sender<Buffer>,
    pub device: Box<dyn BaseAudioOutputDevice>,
}
