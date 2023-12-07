pub struct Range {
    pub min: f32,
    pub max: f32,
}

pub const VALUE_RANGE: Range = Range { min: -1., max: 1. };

pub const FREQUENCY_MIN: f32 = 20.;
pub const FREQUENCY_MAX: f32 = 20_000.;

pub const VALUE_RANGE_WIDTH: f32 = VALUE_RANGE.max - VALUE_RANGE.min;
pub const FREQUENCY_RANGE: f32 = FREQUENCY_MAX - FREQUENCY_MIN;
