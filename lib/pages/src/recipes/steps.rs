use crate::{PageComponent, RenderParameters};
use html::{
    attribute::Attribute,
    elements::{Div, HeaderSize, Headline, HtmlElement, Li, Ol},
};
use recipes::{StepSection, TextBlock};
use std::rc::Rc;

fn render_step_li(step: TextBlock, params: &mut RenderParameters) -> Li {
    Li {
        attributes: vec![],
        content: Rc::new(step.render(params)),
    }
}
impl PageComponent for StepSection {
    fn render(self, date_format: &mut RenderParameters) -> HtmlElement {
        let step_ol = Ol {
            attributes: vec![],
            items: self
                .steps
                .into_iter()
                .map(|step| render_step_li(step, date_format))
                .collect(),
        };
        if self.header.is_empty() {
            step_ol.into()
        } else {
            Div {
                attributes: vec![
                    Attribute::Id(self.header.clone()),
                    Attribute::Class(vec!["level2".to_owned()]),
                ],
                content: Rc::new(
                    vec![
                        Headline {
                            attributes: vec![],
                            size: HeaderSize::H3,
                            content: Rc::new(self.header.into()),
                        }
                        .into(),
                        step_ol.into(),
                    ]
                    .into(),
                ),
            }
            .into()
        }
    }
}
