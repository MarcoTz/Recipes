use super::Measurement;
use std::fmt;

pub struct Ingredient {
    pub measure: Measurement,
    pub ingredient: String,
}

#[derive(Default)]
pub struct IngredientSection {
    pub header: String,
    pub ingredients: Vec<Ingredient>,
}

impl From<Vec<Ingredient>> for IngredientSection {
    fn from(ingredients: Vec<Ingredient>) -> IngredientSection {
        IngredientSection {
            header: "".to_owned(),
            ingredients,
        }
    }
}

impl fmt::Display for Ingredient {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.measure, self.ingredient)
    }
}
impl fmt::Display for IngredientSection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ingredient_strs: Vec<String> = self
            .ingredients
            .iter()
            .map(|ingredient| format!("* {ingredient}"))
            .collect();
        if !self.header.is_empty() {
            writeln!(f, "### {}", self.header)?;
            writeln!(f)?;
        }
        writeln!(f, "{}", ingredient_strs.join("\n"))
    }
}
