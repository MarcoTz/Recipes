use super::{errors::Error, parse_steps::ParseStep};
use recipes::{Ingredient, Measurement, Unit};
use std::str::Split;

pub fn parse_ingredient(input: String) -> Result<Ingredient, Error> {
    let input = input.replace('*', "").trim().to_owned();
    let mut parts = input.split(" ");
    let measurement = parse_measurement(&mut parts)?;
    let ingredient = parts
        .map(|s| s.to_owned())
        .collect::<Vec<String>>()
        .join(" ");
    Ok(Ingredient {
        measure: measurement,
        ingredient,
    })
}

fn parse_measurement(parts: &mut Split<'_, &str>) -> Result<Measurement, Error> {
    let amount_str = parts.next().ok_or(Error::MissingPart(
        "Amount".to_owned(),
        ParseStep::Ingredients,
    ))?;
    let (amount_str, mut unit_str) = take_num(amount_str.to_owned());
    let amount = amount_str
        .parse::<f32>()
        .map_err(|_| Error::Parse("amount".to_owned(), ParseStep::Ingredients))?;
    if unit_str.is_empty() {
        unit_str = parts
            .next()
            .map(|s| s.to_owned())
            .ok_or(Error::MissingPart(
                "Unit".to_owned(),
                ParseStep::Ingredients,
            ))?;
    }
    let unit = unit_str.parse::<Unit>()?;
    Ok(Measurement { amount, unit })
}

fn take_num(input: String) -> (String, String) {
    let mut num_part = "".to_owned();
    let mut rest = "".to_owned();
    let mut chars = input.chars().into_iter();
    while let Some(ch) = chars.next() {
        if ch.is_digit(10) || ch == '.' {
            num_part.push(ch);
        } else {
            rest.push(ch);
            break;
        }
    }
    println!("remaining: {chars:?}");
    rest.push_str(&chars.collect::<String>());
    (num_part, rest)
}
