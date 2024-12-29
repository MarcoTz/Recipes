use crate::{PageComponent, RenderParameters};
use html::{
    attribute::Attribute,
    elements::{HtmlElement, A},
};
use recipes::Tag;
use std::rc::Rc;

impl PageComponent for Tag {
    fn render(self, _: &mut RenderParameters) -> HtmlElement {
        A {
            attributes: vec![Attribute::Href(self.get_url("tags"))],
            content: Rc::new(self.to_string().into()),
        }
        .into()
    }
}
