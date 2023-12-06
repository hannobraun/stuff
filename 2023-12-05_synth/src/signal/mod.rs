mod audio;
mod frequency;
mod signal;
mod source;
mod value;

pub use self::{
    audio::Audio, frequency::Frequency, signal::Signal, value::Control,
};

pub const SAMPLE_RATE: u32 = 48000;
