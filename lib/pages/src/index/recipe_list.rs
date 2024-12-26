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
        let mut recipe_divs = vec![];
        for recipe in self.recipes {
            let mut tag_list: Vec<HtmlElement> = vec!["Tags:".to_owned().into()];
            tag_list.extend(
                recipe
                    .get_tag_urls("tags")
                    .into_iter()
                    .map(|(tag_name, tag_url)| {
                        A {
                            attributes: vec![Attribute::Href(tag_url)],
                            content: Rc::new(tag_name.into()),
                        }
                        .into()
                    })
                    .collect::<Vec<HtmlElement>>(),
            );

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
