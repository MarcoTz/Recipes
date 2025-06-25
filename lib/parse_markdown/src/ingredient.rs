use super::{errors::Error, Parse};
use recipes::{Ingredient, Measurement, TextBlock};

impl Parse for Ingredient {
    fn parse(input: &mut String) -> Result<Ingredient, Error> {
        *input = input.replace('*', "").replace("-", "").trim().to_owned();
        let measurement: Measurement = Parse::parse(input)?;
        let ingredient = input.trim().parse::<TextBlock>()?;
        Ok(Ingredient {
            measure: measurement,
            ingredient,
        })
    }
}
