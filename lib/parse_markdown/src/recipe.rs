use super::{errors::Error, ingredient::parse_ingredient, parse_steps::ParseStep};
use recipes::Recipe;
use std::str::Lines;

pub fn parse_recipe(input: String) -> Result<Recipe, Error> {
    let input = input.trim().to_owned();
    let mut inputs = input.lines();

    let name = parse_name(&mut inputs)?;
    let mut current_step = ParseStep::default();
    let mut ingredients = vec![];
    let mut steps = vec![];
    let mut notes = vec![];
    let mut tags = vec![];

    for mut input in inputs {
        input = input.trim();
        if input.is_empty() {
            continue;
        }
        if input.starts_with("## ") {
            current_step = current_step.next();
            continue;
        }

        match current_step {
            ParseStep::Name => return Err(Error::MissingHeader("name".to_owned())),
            ParseStep::Ingredients => {
                let ing = parse_ingredient(input.to_owned())?;
                ingredients.push(ing);
            }
            ParseStep::Steps => {
                let step = parse_step(input.to_owned())?;
                steps.push(step);
            }
            ParseStep::Notes => notes.push(input.trim().to_owned()),
            ParseStep::Tags => tags.extend(parse_tags(input.to_owned())),
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

fn parse_step(input: String) -> Result<String, Error> {
    println!("current input {input}");
    let step = input
        .split_once(".")
        .map(|res| res.1)
        .ok_or(Error::MissingPart("Number".to_owned(), ParseStep::Steps))?;
    Ok(step.to_owned())
}

fn parse_tags(input: String) -> Vec<String> {
    input
        .split(",")
        .map(|tag| tag.trim().to_owned())
        .collect::<Vec<String>>()
}

fn parse_name(lines: &mut Lines<'_>) -> Result<String, Error> {
    let mut name = lines
        .next()
        .map(|s| s.to_owned())
        .ok_or(Error::MissingHeader("Name".to_owned()))?;
    name = name.replace('#', "");
    Ok(name)
}
