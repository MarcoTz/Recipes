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

impl RecipeList {
    pub fn new(recipes: Vec<Recipe>) -> RecipeList {
        RecipeList { recipes }
    }
}

impl PageComponent for RecipeList {
    fn render(self, date_format: &str) -> HtmlElement {
        let mut recipe_divs = vec![];
        for recipe in self.recipes {
            let mut tag_list: Vec<HtmlElement> = vec!["Tags:".to_owned().into()];
            tag_list.push(recipe.tags.clone().render(date_format));

            let div = Div {
                attributes: vec![Attribute::Class(vec!["recipe_item".to_owned()])],
                content: Rc::new(
                    vec![
                        A {
                            attributes: vec![Attribute::Href(recipe.get_url("recipes"))],
                            content: Rc::new(recipe.name.into()),
                        }
                        .into(),
                        Div {
                            attributes: vec![Attribute::Class(vec!["recipe_taglist".to_owned()])],
                            content: Rc::new(tag_list.into()),
                        }
                        .into(),
                    ]
                    .into(),
                ),
            }
            .into();
            recipe_divs.push(div);
        }
        Div {
            attributes: vec![Attribute::Id("recipe_list".to_owned())],
            content: Rc::new(recipe_divs.into()),
        }
        .into()
    }
}
