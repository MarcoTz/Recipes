use super::{Measurement, Tag};
use std::fmt;

pub struct Recipe {
    name: String,
    ingredients: Vec<(Measurement, String)>,
    steps: Vec<String>,
    notes: Vec<String>,
    tags: Vec<Tag>,
}

impl fmt::Display for Recipe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ingredient_strs: Vec<String> = self
            .ingredients
            .iter()
            .map(|(measure, ingredient)| format!("* {measure} {ingredient}"))
            .collect();
        let step_strs: Vec<String> = self
            .steps
            .iter()
            .enumerate()
            .map(|(num, step)| format!("{num}. {step}"))
            .collect();
        write!(
            f,
            "#{}

            ## Ingredients

            {}

            ## Steps 
            
            {}

            ## Notes

            {}

            ## Tags
            {}
            ",
            self.name,
            ingredient_strs.join("\n"),
            step_strs.join("\n"),
            self.notes.join("\n\n"),
            self.tags.join(",")
        )
    }
}
