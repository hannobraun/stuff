pub const SAMPLE_RATE: usize = 48_000;
pub const SAMPLE_COUNT: usize = SAMPLE_RATE / 20; // results in 50 ms latency

pub const NUM_CHANNELS: usize = 1;

pub const BUFFER_SIZE: usize = SAMPLE_COUNT * NUM_CHANNELS;
pub type Buffer = [f32; BUFFER_SIZE];
