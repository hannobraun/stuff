use crate::{
    range::Range,
    signal::{Signal, Value},
};

pub trait Limit {
    fn limit(
        self,
        min: impl Into<Signal>,
        max: impl Into<Signal>,
        range: Range,
    ) -> Signal;
}

impl Limit for Signal {
    fn limit(
        mut self,
        min: impl Into<Signal>,
        max: impl Into<Signal>,
        range: Range,
    ) -> Signal {
        let mut min = min.into();
        let mut max = max.into();

        Signal::from_fn(move || {
            let value = self.next_value().decode_to(range);

            let min = min.next_value().decode_to(range);
            let max = max.next_value().decode_to(range);

            let value = value.clamp(min, max);
            Value::encode_from(value, range)
        })
    }
}
