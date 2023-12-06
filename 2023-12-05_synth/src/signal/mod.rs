mod audio;
mod signal;
mod source;
mod value;

pub use self::{audio::Audio, signal::Signal, value::Value};

pub const SAMPLE_RATE: u32 = 48000;
