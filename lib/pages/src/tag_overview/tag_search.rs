pub struct TagSearch;
use crate::PageComponent;
use html::{
    attribute::Attribute,
    elements::{Div, HtmlElement, Input},
};
use std::rc::Rc;

impl PageComponent for TagSearch {
    fn render(self, _: &str) -> HtmlElement {
        Div {
            attributes: vec![Attribute::Id("searchbox".to_owned())],
            content: Rc::new(
                vec![
                    "Filter".to_owned().into(),
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
