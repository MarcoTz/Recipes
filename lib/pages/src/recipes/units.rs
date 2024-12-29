use crate::PageComponent;
use html::elements::HtmlElement;
use recipes::Unit;

impl PageComponent for Unit {
    fn render(self, _: &str) -> HtmlElement {
        self.to_string().replace('_', "").into()
    }
}
