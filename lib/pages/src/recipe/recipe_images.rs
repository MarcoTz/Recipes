use crate::PageComponent;
use html::{
    attribute::Attribute,
    elements::{Div, HtmlElement, Img},
};
use std::rc::Rc;

pub struct RecipeImages {
    pub image_urls: Vec<String>,
}

impl PageComponent for RecipeImages {
    fn render(self, _: &str) -> HtmlElement {
        let img_divs: Vec<HtmlElement> = self
            .image_urls
            .into_iter()
            .map(|url| {
                Div {
                    attributes: vec![Attribute::Class(vec!["recipe_image".to_owned()])],
                    content: Rc::new(
                        Img {
                            attributes: vec![Attribute::Src(url)],
                        }
                        .into(),
                    ),
                }
                .into()
            })
            .collect();
        Div {
            attributes: vec![Attribute::Id("recipe_images".to_owned())],
            content: Rc::new(img_divs.into()),
        }
        .into()
    }
}
