use super::errors::Error;
use std::{fmt, str::FromStr};

pub enum Unit {
    Gram,
    Liter,
    Milliliter,
    Teaspoon,
    Tablespoon,
    Piece,
    // add more as apropriate
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Unit::Gram => f.write_str("g"),
            Unit::Liter => f.write_str("l"),
            Unit::Milliliter => f.write_str("ml"),
            Unit::Teaspoon => f.write_str("tsp"),
            Unit::Tablespoon => f.write_str("Tbsp"),
            Unit::Piece => f.write_str(""),
        }
    }
}

impl FromStr for Unit {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().trim() {
            "g" | "gram" => Ok(Unit::Gram),
            "l" | "liter" => Ok(Unit::Liter),
            "tbsp" | "tablespoon" => Ok(Unit::Tablespoon),
            "ml" | "milliliter" => Ok(Unit::Milliliter),
            "tsp" | "teaspoon" => Ok(Unit::Teaspoon),
            inp => Err(Error::UnknownUnit(inp.to_owned())),
        }
    }
}
