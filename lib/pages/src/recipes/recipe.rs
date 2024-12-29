use crate::PageComponent;
use html::{
    attribute::Attribute,
    elements::{Div, HeaderSize, Headline, HtmlElement, Li, Ol, A},
};
use recipes::{Recipe, StepSection, Tag, TextBlock};
use std::rc::Rc;

fn render_step_li(step: TextBlock, date_format: &str) -> Li {
    Li {
        attributes: vec![],
        content: Rc::new(step.render(date_format)),
    }
}

impl PageComponent for Tag {
    fn render(self, _: &str) -> HtmlElement {
        A {
            attributes: vec![Attribute::Href(self.get_url("../tags"))],
            content: Rc::new(self.to_string().into()),
        }
        .into()
    }
}

impl PageComponent for StepSection {
    fn render(self, date_format: &str) -> HtmlElement {
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

impl PageComponent for Recipe {
    fn render(self, date_format: &str) -> HtmlElement {
        Div {
            attributes: vec![
                Attribute::Id(self.name.clone()),
                Attribute::Class(vec!["level1".to_owned()]),
            ],
            content: Rc::new(
                vec![
                    Headline {
                        attributes: vec![],
                        size: HeaderSize::H1,
                        content: Rc::new(self.name.into()),
                    }
                    .into(),
                    Div {
                        attributes: vec![
                            Attribute::Id("ingredients".to_owned()),
                            Attribute::Class(vec!["level2".to_owned()]),
                        ],
                        content: Rc::new(self.ingredients.render(date_format)),
                    }
                    .into(),
                    Div {
                        attributes: vec![
                            Attribute::Id("steps".to_owned()),
                            Attribute::Class(vec!["level2".to_owned()]),
                        ],
                        content: Rc::new(self.steps.render(date_format)),
                    }
                    .into(),
                    Div {
                        attributes: vec![
                            Attribute::Id("notes".to_owned()),
                            Attribute::Class(vec!["level2".to_owned()]),
                        ],
                        content: Rc::new(self.notes.render(date_format).into()),
                    }
                    .into(),
                    Div {
                        attributes: vec![
                            Attribute::Id("tags".to_owned()),
                            Attribute::Class(vec!["level2".to_owned()]),
                        ],
                        content: Rc::new(self.tags.render(date_format)),
                    }
                    .into(),
                ]
                .into(),
            ),
        }
        .into()
    }
}
