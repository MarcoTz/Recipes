use crate::PageComponent;
use html::elements::HtmlElement;
use recipes::Note;

impl PageComponent for Note {
    fn render(self, date_format: &str) -> HtmlElement {
        vec![self.0.render(date_format), HtmlElement::Br].into()
    }
}
