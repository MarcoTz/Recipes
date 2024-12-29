use crate::{PageComponent, RenderParameters};
use html::elements::HtmlElement;
use recipes::{Amount, Measurement};

impl PageComponent for Amount {
    fn render(self, _: &mut RenderParameters) -> HtmlElement {
        match self {
            Amount::Num(n) => n.to_string().into(),
            Amount::Range(from, to) => format!("{from}-{to}").into(),
        }
    }
}

impl PageComponent for Measurement {
    fn render(self, date_format: &mut RenderParameters) -> HtmlElement {
        vec![
            self.amount.render(date_format),
            self.unit.to_string().replace("_", "").into(),
        ]
        .into()
    }
}
