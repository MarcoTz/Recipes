use super::{PageComponent, RenderParameters};
use html::{
    attribute::Attribute,
    elements::{Div, HtmlElement, A},
};
use std::rc::Rc;

pub struct Header;

impl PageComponent for Header {
    fn render(self, params: &mut RenderParameters) -> HtmlElement {
        Div {
            attributes: vec![Attribute::Id("header".to_owned())],
            content: Rc::new(
                vec![
                    Div {
                        attributes: vec![Attribute::Id("headline".to_owned())],
                        content: Rc::new(
                            A {
                                attributes: vec![Attribute::Href(
                                    params.get_link_prefix() + "index.html",
                                )],
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
                                attributes: vec![Attribute::Href(
                                    params.get_link_prefix() + "tag_overview.html",
                                )],
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
