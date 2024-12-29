use super::{IngredientSection, TextBlock};
use std::fmt;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct StepSection {
    pub header: String,
    pub steps: Vec<TextBlock>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Tag(pub String);

impl Tag {
    pub fn get_url(&self, base: &str) -> String {
        format!("{base}/{}.html", self.0.replace(" ", ""))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Recipe {
    pub name: String,
    pub ingredients: Vec<IngredientSection>,
    pub steps: Vec<StepSection>,
    pub notes: Vec<TextBlock>,
    pub tags: Vec<Tag>,
    pub image_filenames: Vec<String>,
}

impl Recipe {
    pub fn get_url(&self, base: &str) -> String {
        format!("{base}/{}.html", self.name.replace(" ", ""))
    }
}

impl From<Vec<TextBlock>> for StepSection {
    fn from(steps: Vec<TextBlock>) -> StepSection {
        StepSection {
            header: "".to_owned(),
            steps,
        }
    }
}

impl fmt::Display for StepSection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let step_strs: Vec<String> = self
            .steps
            .iter()
            .enumerate()
            .map(|(num, step)| format!("{}. {step}", num + 1))
            .collect();
        if !self.header.is_empty() {
            writeln!(f, "### {}", self.header)?;
            writeln!(f)?;
        }
        writeln!(f, "{}", step_strs.join("\n"))
    }
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.0)
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
