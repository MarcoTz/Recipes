use super::{IngredientSection, Note, StepSection, Tag};
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub struct Recipe {
    pub name: String,
    pub ingredients: Vec<IngredientSection>,
    pub steps: Vec<StepSection>,
    pub notes: Vec<Note>,
    pub tags: Vec<Tag>,
    pub image_filenames: Vec<String>,
}

impl Recipe {
    pub fn get_url(&self, base: &str) -> String {
        format!("{base}/{}.html", self.name.replace(" ", ""))
    }
}

impl fmt::Display for Recipe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ingredient_strs: Vec<String> = self
            .ingredients
            .iter()
            .map(|sec| format!("{sec}"))
            .collect();
        let step_strs: Vec<String> = self.steps.iter().map(|sec| format!("{sec}")).collect();
        let note_strs: Vec<String> = self.notes.iter().map(|note| note.to_string()).collect();

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
        writeln!(f, "{}", note_strs.join("\n\n"))?;
        writeln!(f,)?;
        writeln!(f, "## Tags")?;
        writeln!(
            f,
            "{}",
            self.tags
                .iter()
                .map(|tag| tag.to_string())
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}
