use super::{errors::Error, parse_steps::ParseStep};
use recipes::TextBlock;

pub fn parse_step(input: String) -> Result<TextBlock, Error> {
    let step = input
        .split_once(".")
        .map(|res| res.1.trim())
        .ok_or(Error::MissingPart("Number".to_owned(), ParseStep::Steps))?;
    let block = step.parse::<TextBlock>()?;
    Ok(block)
}
