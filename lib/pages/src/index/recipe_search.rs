use crate::{PageComponent, RenderParameters};
use html::{
    attribute::Attribute,
    elements::{Div, HtmlElement, Input},
};
use std::rc::Rc;

pub struct RecipeSearch;

impl PageComponent for RecipeSearch {
    fn render(self, _: &mut RenderParameters) -> HtmlElement {
        Div {
            attributes: vec![Attribute::Id("searchbox".to_owned())],
            content: Rc::new(
                vec![
                    "filter".to_owned().into(),
                    Input {
                        attributes: vec![
                            Attribute::Type("text".to_owned()),
                            Attribute::Id("search".to_owned()),
                            Attribute::OnKeyUp("filter()".to_owned()),
                        ],
                    }
                    .into(),
                ]
                .into(),
            ),
        }
        .into()
    }
}
