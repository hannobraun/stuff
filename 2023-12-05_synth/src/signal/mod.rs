pub mod range;

mod signal;
mod source;
mod value;

pub use self::{
    range::{AMPLIFIER_RANGE, AUDIBLE_RANGE},
    signal::Signal,
    value::Value,
};

pub const SAMPLE_RATE: u32 = 48000;
