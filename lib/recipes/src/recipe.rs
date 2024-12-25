use super::{Measurement, Tag};
use std::fmt;

pub struct Recipe {
    pub name: String,
    pub ingredients: Vec<(Measurement, String)>,
    pub steps: Vec<String>,
    pub notes: Vec<String>,
    pub tags: Vec<Tag>,
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
            .map(|(num, step)| format!("{}. {step}", num + 1))
            .collect();
        writeln!(f, "# {}", self.name)?;
        writeln!(f,)?;
        writeln!(f, "## Ingredients")?;
        writeln!(f,)?;
        writeln!(f, "{}", ingredient_strs.join("\n"))?;
        writeln!(f,)?;
        writeln!(f, "## Steps")?;
        writeln!(f,)?;
        writeln!(f, "{}", step_strs.join("\n"))?;
        writeln!(f,)?;
        writeln!(f, "## Notes")?;
        writeln!(f,)?;
        writeln!(f, "{}", self.notes.join("\n\n"))?;
        writeln!(f,)?;
        writeln!(f, "## Tags")?;
        writeln!(f, "{}", self.tags.join(","))
    }
}
