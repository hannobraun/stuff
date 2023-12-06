mod audio;
mod control;
mod frequency;
mod signal;
mod source;

pub use self::{
    audio::Audio, control::Control, frequency::Frequency, signal::Signal,
};

pub const SAMPLE_RATE: u32 = 48000;
