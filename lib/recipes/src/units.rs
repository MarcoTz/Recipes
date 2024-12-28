use super::errors::Error;
use std::{fmt, str::FromStr};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Unit {
    Gram,
    Kilogram,
    Liter,
    Milliliter,
    Teaspoon,
    Tablespoon,
    Piece,
    Centimeter, // add more as apropriate
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Unit::Gram => f.write_str("g"),
            Unit::Kilogram => f.write_str("kg"),
            Unit::Liter => f.write_str("l"),
            Unit::Milliliter => f.write_str("ml"),
            Unit::Teaspoon => f.write_str(" tsp"),
            Unit::Tablespoon => f.write_str(" Tbsp"),
            Unit::Piece => f.write_str("_"),
            Unit::Centimeter => f.write_str("cm"),
        }
    }
}

impl FromStr for Unit {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().trim() {
            "g" | "gram" => Ok(Unit::Gram),
            "kg" | "kilogram" => Ok(Unit::Kilogram),
            "l" | "liter" => Ok(Unit::Liter),
            "tbsp" | "tablespoon" => Ok(Unit::Tablespoon),
            "ml" | "milliliter" => Ok(Unit::Milliliter),
            "tsp" | "teaspoon" => Ok(Unit::Teaspoon),
            "cm" | "centimeter" => Ok(Unit::Centimeter),
            "_" => Ok(Unit::Piece),
            inp => Err(Error::UnknownUnit(inp.to_owned())),
        }
    }
}
