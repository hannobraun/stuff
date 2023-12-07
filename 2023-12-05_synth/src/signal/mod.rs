mod control;
mod frequency;
mod signal;
mod source;
mod value;

pub use self::{
    control::Control, frequency::Frequency, signal::Signal, value::Value,
};

pub const SAMPLE_RATE: u32 = 48000;
