use super::Unit;
use std::fmt;

pub struct Measurement {
    pub amount: f32,
    pub unit: Unit,
}

impl fmt::Display for Measurement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.amount, self.unit)
    }
}
