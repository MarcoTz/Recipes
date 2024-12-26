use super::PageComponent;
use chrono::NaiveDate;
use html::{
    attribute::Attribute,
    elements::{Div, HtmlElement, A},
};
use std::rc::Rc;

pub struct Footer {
    pub created_date: NaiveDate,
    pub num_recipes: usize,
}

impl PageComponent for Footer {
    fn render(self, date_format: &str) -> HtmlElement {
        vec![
            Div {
                attributes: vec![Attribute::Class(vec!["spacer".to_owned()])],
                content: Rc::new("".to_owned().into()),
            }
            .into(),
            Div {
                attributes: vec![Attribute::Class(vec!["hline".to_owned()])],
                content: Rc::new("".to_owned().into()),
            }
            .into(),
            Div {
                attributes: vec![Attribute::Id("footer".to_owned())],
                content: Rc::new(
                    vec![
                        Div {
                            attributes: vec![Attribute::Id("github_link".to_owned())],
                            content: Rc::new(
                                A {
                                    attributes: vec![Attribute::Href(
                                        "https://github.com/MarcoTz/Recipes".to_owned(),
                                    )],
                                    content: Rc::new("Github".to_owned().into()),
                                }
                                .into(),
                            ),
                        }
                        .into(),
                        Div {
                            attributes: vec![Attribute::Id("created_date".to_owned())],
                            content: Rc::new(
                                format!("Created At: {}", self.created_date.format(&date_format))
                                    .into(),
                            ),
                        }
                        .into(),
                        Div {
                            attributes: vec![Attribute::Id("num_recipes".to_owned())],
                            content: Rc::new(format!("{}", self.num_recipes).into()),
                        }
                        .into(),
                    ]
                    .into(),
                ),
            }
            .into(),
        ]
        .into()
    }
}
