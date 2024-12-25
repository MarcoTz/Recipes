use super::Measurement;
use std::fmt;

pub struct Ingredient {
    pub measure: Measurement,
    pub ingredient: String,
}

impl fmt::Display for Ingredient {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.measure, self.ingredient)
    }
}
