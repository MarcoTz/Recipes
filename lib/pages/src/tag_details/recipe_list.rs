use crate::PageComponent;
use html::{
    attribute::Attribute,
    elements::{Div, HtmlElement, A},
};
use recipes::Recipe;
use std::rc::Rc;

pub struct RecipeList {
    pub recipes: Vec<Recipe>,
}

impl PageComponent for RecipeList {
    fn render(self, _: &str) -> HtmlElement {
        let recipe_divs: Vec<HtmlElement> = self
            .recipes
            .into_iter()
            .map(|recipe| {
                Div {
                    attributes: vec![Attribute::Class(vec!["recipe_item".to_owned()])],
                    content: Rc::new(
                        A {
                            attributes: vec![Attribute::Href(recipe.get_url("../recipes"))],
                            content: Rc::new(recipe.name.into()),
                        }
                        .into(),
                    ),
                }
                .into()
            })
            .collect();
        Div {
            attributes: vec![Attribute::Id("recipe_list".to_owned())],
            content: Rc::new(recipe_divs.into()),
        }
        .into()
    }
}
