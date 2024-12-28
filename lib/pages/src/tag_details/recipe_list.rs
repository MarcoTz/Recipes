use crate::PageComponent;
use html::{
    attribute::Attribute,
    elements::{Div, HtmlElement, A},
};
use recipes::Recipe;
use std::rc::Rc;

pub struct RecipeList {
    pub recipes: Vec<(String, String)>,
}

impl RecipeList {
    pub fn new(recipes: &[&Recipe]) -> RecipeList {
        let mut recipe_urls = vec![];
        for recipe in recipes {
            let url = recipe.get_url("../recipes");
            recipe_urls.push((recipe.name.clone(), url));
        }
        RecipeList {
            recipes: recipe_urls,
        }
    }
}

impl PageComponent for RecipeList {
    fn render(self, _: &str) -> HtmlElement {
        let recipe_divs: Vec<HtmlElement> = self
            .recipes
            .into_iter()
            .map(|(recipe_name, recipe_url)| {
                Div {
                    attributes: vec![Attribute::Class(vec!["recipe_item".to_owned()])],
                    content: Rc::new(
                        A {
                            attributes: vec![Attribute::Href(recipe_url)],
                            content: Rc::new(recipe_name.into()),
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
