use super::{errors::Error, parse_steps::ParseStep};
use recipes::{Ingredient, Measurement, Unit};

pub fn parse_ingredient(input: String) -> Result<Ingredient, Error> {
    let mut input = input.replace('*', "").trim().to_owned();
    let measurement = parse_measurement(&mut input)?;
    Ok(Ingredient {
        measure: measurement,
        ingredient: input,
    })
}

fn parse_measurement(input: &mut String) -> Result<Measurement, Error> {
    let amount_str = take_num(input);
    let amount = amount_str
        .parse::<f32>()
        .map_err(|_| Error::Parse("Amount".to_owned(), ParseStep::Ingredients))?;
    let unit_str = input
        .split_once(" ")
        .map(|(u, _)| u.to_owned())
        .unwrap_or("".to_owned());
    *input = input.replacen(&unit_str, "", 1);

    let unit = unit_str.parse::<Unit>()?;
    Ok(Measurement { amount, unit })
}

fn take_num(input: &mut String) -> String {
    let mut num_part = "".to_owned();
    while !input.is_empty() {
        let ch = input.remove(0);
        if ch == '-' {
            continue; //TODO amounts "x-y unit" should be allowed
        }
        if ch.is_digit(10) || ch == '.' {
            num_part.push(ch);
        } else {
            input.insert(0, ch);
            break;
        }
    }
    *input = input.trim().to_owned();
    num_part
}
