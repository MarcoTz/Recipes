use super::{errors::Error, parse_steps::ParseStep, Parse};
use recipes::{Amount, Measurement, Unit};

impl Parse for Measurement {
    fn parse(input: &mut String) -> Result<Measurement, Error> {
        let amount: Amount = Parse::parse(input)?;
        let unit_str = input
            .split_once(" ")
            .map(|(u, _)| u.to_owned())
            .unwrap_or("".to_owned());
        *input = input.replacen(&unit_str, "", 1);

        let unit = unit_str.parse::<Unit>()?;
        Ok(Measurement { amount, unit })
    }
}

impl Parse for Amount {
    fn parse(input: &mut String) -> Result<Amount, Error> {
        let mut num_first = "".to_owned();
        let mut num_second = "".to_owned();
        let mut first = true;
        while !input.is_empty() {
            let ch = input.remove(0);
            if ch == '-' {
                first = false;
                continue;
            }
            if !ch.is_ascii_digit() && ch != '.' {
                input.insert(0, ch);
                break;
            }
            if first {
                num_first.push(ch);
            } else {
                num_second.push(ch);
            }
        }

        *input = input.trim().to_owned();
        let from = num_first
            .parse::<f32>()
            .map_err(|_| Error::Parse("Amount".to_owned(), ParseStep::Ingredients))?;
        if num_second.is_empty() {
            Ok(from.into())
        } else {
            let to = num_second
                .parse::<f32>()
                .map_err(|_| Error::Parse("Amount".to_owned(), ParseStep::Ingredients))?;
            Ok((from, to).into())
        }
    }
}
