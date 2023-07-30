pub trait Signal: Send {
    // The way this is currently used, `next_value` is called and that moves the
    // signal forward. This is not good, once the same signal can be used as
    // input in multiple places. We don't want multiple calls to this to advance
    // the signal multiple times!
    //
    // Probably best to make this take `&self` and a global clock value. Then
    // everything is pure, and we don't have to worry about stuff like that.
    fn next_value(&mut self) -> f32;
}

pub struct Value(pub f32);

impl Signal for Value {
    fn next_value(&mut self) -> f32 {
        self.0
    }
}
