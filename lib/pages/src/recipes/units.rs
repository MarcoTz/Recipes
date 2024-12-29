use crate::{PageComponent, RenderParameters};
use html::elements::HtmlElement;
use recipes::Unit;

impl PageComponent for Unit {
    fn render(self, _: &mut RenderParameters) -> HtmlElement {
        self.to_string().replace('_', "").into()
    }
}
