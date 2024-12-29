use crate::PageComponent;
use html::elements::HtmlElement;
use recipes::Recipe;

#[derive(PartialEq)]
pub struct RecipeContents {
    pub recipe: Recipe,
}

impl RecipeContents {
    pub fn new(recipe: &Recipe) -> RecipeContents {
        RecipeContents {
            recipe: recipe.clone(),
        }
    }
}

impl PageComponent for RecipeContents {
    fn render(self, date_format: &str) -> HtmlElement {
        self.recipe.render(date_format)
    }
}
