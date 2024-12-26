use super::{IngredientSection, Tag};
use std::{fmt, path::PathBuf};

#[derive(Debug, Default, PartialEq, Eq)]
pub struct StepSection {
    pub header: String,
    pub steps: Vec<String>,
}

#[derive(Debug, PartialEq)]
pub struct Recipe {
    pub name: String,
    pub ingredients: Vec<IngredientSection>,
    pub steps: Vec<StepSection>,
    pub notes: Vec<String>,
    pub tags: Vec<Tag>,
    pub images: Vec<PathBuf>,
}

impl Recipe {
    pub fn get_url(&self, base: &str) -> String {
        format!("{base}/{}.html", self.name.replace(" ", ""))
    }

    pub fn get_tag_urls(&self, base: &str) -> Vec<(String, String)> {
        let mut urls = vec![];
        for tag in self.tags.iter() {
            urls.push((tag.clone(), format!("{base}/{}.html", tag.replace(" ", ""))));
        }
        urls
    }
}

impl From<Vec<String>> for StepSection {
    fn from(steps: Vec<String>) -> StepSection {
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

impl fmt::Display for Recipe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ingredient_strs: Vec<String> = self
            .ingredients
            .iter()
            .map(|sec| format!("{sec}"))
            .collect();
        let step_strs: Vec<String> = self.steps.iter().map(|sec| format!("{sec}")).collect();
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
