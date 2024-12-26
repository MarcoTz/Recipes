use super::PageComponent;
use html::{
    attribute::Attribute,
    elements::{Div, HtmlElement, A},
};
use std::rc::Rc;

pub struct Header {
    pub index_link: String,
    pub tag_link: String,
}

impl PageComponent for Header {
    fn render(self, _: &str) -> HtmlElement {
        Div {
            attributes: vec![Attribute::Id("header".to_owned())],
            content: Rc::new(
                vec![
                    Div {
                        attributes: vec![Attribute::Id("headline".to_owned())],
                        content: Rc::new(
                            A {
                                attributes: vec![Attribute::Href(self.index_link)],
                                content: Rc::new("All Recipes".to_owned().into()),
                            }
                            .into(),
                        ),
                    }
                    .into(),
                    Div {
                        attributes: vec![Attribute::Id("tagslink".to_owned())],
                        content: Rc::new(
                            A {
                                attributes: vec![Attribute::Href(self.tag_link)],
                                content: Rc::new("Tags".to_owned().into()),
                            }
                            .into(),
                        ),
                    }
                    .into(),
                ]
                .into(),
            ),
        }
        .into()
    }
}
