use crate::{PageComponent, RenderParameters};
use html::{
    attribute::Attribute,
    elements::{HtmlElement, A},
};
use recipes::{TextBlock, TextElement};
use std::rc::Rc;

impl PageComponent for TextBlock {
    fn render(self, date_format: &mut RenderParameters) -> HtmlElement {
        self.elements
            .into_iter()
            .map(|elem| elem.render(date_format))
            .collect::<Vec<HtmlElement>>()
            .into()
    }
}

impl PageComponent for TextElement {
    fn render(self, _: &mut RenderParameters) -> HtmlElement {
        match self {
            TextElement::Link { label, target } => A {
                attributes: vec![Attribute::Href(target)],
                content: Rc::new(label.into()),
            }
            .into(),
            TextElement::Plain(text) => text.into(),
        }
    }
}
