use crate::{PageComponent, RenderParameters};
use html::elements::HtmlElement;
use recipes::Note;

impl PageComponent for Note {
    fn render(self, params: &mut RenderParameters) -> HtmlElement {
        vec![self.0.render(params), HtmlElement::Br].into()
    }
}
