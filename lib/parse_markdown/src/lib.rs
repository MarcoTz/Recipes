pub mod errors;
pub mod parse_steps;

use errors::Error;
use parse_steps::ParseStep;
use recipes::{Ingredient, Measurement, Recipe, Unit};
use std::{
    ffi::OsString,
    fs::{read_dir, read_to_string},
    path::PathBuf,
    str::FromStr,
};

pub fn load_markdown(path: PathBuf) -> Result<Vec<String>, Error> {
    let dir_contents = read_dir(path)?;
    let mut contents = vec![];
    for m_entry in dir_contents {
        let entry = m_entry?;
        let path = entry.path();
        if path.extension() == Some(&OsString::from_str("md").unwrap()) {
            contents.push(read_to_string(entry.path())?);
        }
    }
    Ok(contents)
}

pub fn parse_recipe(input: String) -> Result<Recipe, Error> {
    let input = input.trim().to_owned();
    let mut lines = input.lines();
    let mut name = lines
        .next()
        .map(|s| s.to_owned())
        .ok_or(Error::MissingHeader("Name".to_owned()))?;
    name = name.replace('#', "");

    let mut current_step = ParseStep::default();
    let mut ingredients = vec![];
    let mut steps = vec![];
    let mut notes = vec![];
    let mut tags = vec![];

    for mut line in lines {
        line = line.trim();
        if line.is_empty() {
            continue;
        }
        if line.starts_with("## ") {
            current_step = current_step.next();
            println!("switched step to {current_step}, line {line}");
            continue;
        }

        match current_step {
            ParseStep::Name => return Err(Error::MissingHeader("name".to_owned())),
            ParseStep::Ingredients => {
                let line_repl = line.replace('*', "");
                println!("line for ingredients {line}");
                line = line_repl.trim();
                let mut parts = line.split(" ");
                println!(
                    "parts for ingredients: {:?}",
                    parts.clone().collect::<Vec<&str>>()
                );

                let amount_str = parts.next().ok_or(Error::MissingPart(
                    "Amount".to_owned(),
                    current_step.clone(),
                ))?;
                let (amount_str, mut unit_str) = take_num(amount_str.to_owned());
                println!("after split {unit_str}");
                println!("amounts {amount_str}");
                let amount = amount_str
                    .parse::<f32>()
                    .map_err(|_| Error::Parse("amount".to_owned(), current_step.clone()))?;
                if unit_str.is_empty() {
                    unit_str = parts
                        .next()
                        .map(|s| s.to_owned())
                        .ok_or(Error::MissingPart("Unit".to_owned(), current_step.clone()))?;
                }
                println!("unit {unit_str}");
                let unit = unit_str.parse::<Unit>()?;
                let ingredient = parts
                    .map(|s| s.to_owned())
                    .collect::<Vec<String>>()
                    .join(" ");
                ingredients.push(Ingredient {
                    measure: Measurement { amount, unit },
                    ingredient,
                });
            }
            ParseStep::Steps => {
                println!("current line {line}");
                line = line
                    .split_once(".")
                    .map(|res| res.1)
                    .ok_or(Error::MissingPart(
                        "Number".to_owned(),
                        current_step.clone(),
                    ))?;
                steps.push(line.to_owned())
            }
            ParseStep::Notes => notes.push(line.trim().to_owned()),
            ParseStep::Tags => {
                tags.extend(
                    line.split(",")
                        .map(|tag| tag.trim().to_owned())
                        .collect::<Vec<String>>(),
                );
            }
            ParseStep::Done => break,
        }
    }
    Ok(Recipe {
        name,
        ingredients,
        steps,
        notes,
        tags,
    })
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
