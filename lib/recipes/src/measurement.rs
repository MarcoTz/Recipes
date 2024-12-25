use super::Unit;
use std::fmt;

pub enum Amount {
    Num(f32),
    Range(f32, f32),
}

pub struct Measurement {
    pub amount: Amount,
    pub unit: Unit,
}

impl From<f32> for Amount {
    fn from(num: f32) -> Amount {
        Amount::Num(num)
    }
}

impl From<(f32, f32)> for Amount {
    fn from((from, to): (f32, f32)) -> Amount {
        Amount::Range(from, to)
    }
}

impl fmt::Display for Measurement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.amount, self.unit)
    }
}

impl fmt::Display for Amount {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Amount::Num(num) => write!(f, "{num}"),
            Amount::Range(from, to) => write!(f, "{from}-{to}"),
        }
    }
}
