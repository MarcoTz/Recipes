use super::{errors::Error, ingredient::parse_ingredient, parse_steps::ParseStep};
use recipes::{IngredientSection, Recipe, StepSection};
use std::str::Lines;

pub fn parse_recipe(input: String) -> Result<Recipe, Error> {
    let input = input.trim().to_owned();
    let mut inputs = input.lines();

    let name = parse_name(&mut inputs)?;
    let mut current_step = ParseStep::default();
    let mut previous_ingredients = vec![];
    let mut current_ingredients = IngredientSection::default();
    let mut previous_steps = vec![];
    let mut current_steps = StepSection::default();
    let mut notes = vec![];
    let mut tags = vec![];
    for mut input in inputs {
        input = input.trim();
        if input.starts_with("## ") {
            current_step = current_step.next();
            continue;
        }

        if input.is_empty() || input.starts_with('#') {
            let next_header = input.replace('#', "").trim().to_owned();
            if current_step == ParseStep::Ingredients {
                previous_ingredients.push(current_ingredients);
                current_ingredients = IngredientSection::default();
                current_ingredients.header = next_header;
            } else if current_step == ParseStep::Steps {
                previous_steps.push(current_steps);
                current_steps = StepSection::default();
                current_steps.header = next_header;
            }
            continue;
        }

        match current_step {
            ParseStep::Name => return Err(Error::MissingHeader("name".to_owned())),
            ParseStep::Ingredients => {
                let ing = parse_ingredient(input.to_owned())?;
                current_ingredients.ingredients.push(ing);
            }
            ParseStep::Steps => {
                let step = parse_step(input.to_owned())?;
                current_steps.steps.push(step);
            }
            ParseStep::Notes => notes.push(input.trim().to_owned()),
            ParseStep::Tags => tags.extend(parse_tags(input.to_owned())),
            ParseStep::Done => break,
        }
    }
    previous_ingredients.push(current_ingredients);
    previous_steps.push(current_steps);
    Ok(Recipe {
        name,
        ingredients: previous_ingredients,
        steps: previous_steps,
        notes,
        tags,
    })
}

fn parse_step(input: String) -> Result<String, Error> {
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
