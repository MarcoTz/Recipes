use crate::PageComponent;
use html::{
    attribute::Attribute,
    elements::{Div, HeaderSize, Headline, HtmlElement},
};
use recipes::Recipe;
use std::rc::Rc;

impl PageComponent for Recipe {
    fn render(self, date_format: &str) -> HtmlElement {
        let notes_rendered = if self.notes.is_empty() {
            "".to_owned().into()
        } else {
            vec![
                Headline {
                    size: HeaderSize::H3,
                    attributes: vec![],
                    content: Rc::new("Notes".to_owned().into()),
                }
                .into(),
                self.notes.render(date_format),
            ]
            .into()
        };

        let tags_rendered = if self.tags.is_empty() {
            "".to_owned().into()
        } else {
            vec![
                Headline {
                    size: HeaderSize::H3,
                    attributes: vec![],
                    content: Rc::new("Tags".to_owned().into()),
                }
                .into(),
                self.tags.render(date_format),
            ]
            .into()
        };

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
                        content: Rc::new(notes_rendered),
                    }
                    .into(),
                    Div {
                        attributes: vec![
                            Attribute::Id("tags".to_owned()),
                            Attribute::Class(vec!["level2".to_owned()]),
                        ],
                        content: Rc::new(tags_rendered),
                    }
                    .into(),
                ]
                .into(),
            ),
        }
        .into()
    }
}
