use crate::{PageComponent, RenderParameters};
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
    fn render(self, params: &mut RenderParameters) -> HtmlElement {
        params.depth = 1;
        self.recipe.render(params)
    }
}
