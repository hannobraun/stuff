mod audio;
mod control;
mod frequency;
mod signal;
mod source;

pub use self::{
    audio::Value, control::Control, frequency::Frequency, signal::Signal,
};

pub const SAMPLE_RATE: u32 = 48000;
