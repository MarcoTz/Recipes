use std::fmt;

pub enum Unit {
    Gram,
    Liter, // add more as apropriate
}

pub struct Measurement {
    pub amount: f32,
    pub unit: Unit,
}

impl fmt::Display for Measurement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.amount, self.unit)
    }
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Unit::Gram => f.write_str("g"),
            Unit::Liter => f.write_str("l"),
        }
    }
}
