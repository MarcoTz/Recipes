use super::PageComponent;
use html::{
    attribute::Attribute,
    elements::{Head, HtmlElement, Link, Script},
};
use std::rc::Rc;

pub struct HtmlHead {
    pub title: String,
}

impl HtmlHead {
    pub fn as_head(self) -> Head {
        Head {
            title: self.title,
            content: Rc::new(
                vec![
                    Link {
                        attributes: vec![
                            Attribute::Rel("stylesheet".to_owned()),
                            Attribute::Href("main.css".to_owned()),
                        ],
                    }
                    .into(),
                    Script {
                        attributes: vec![Attribute::Src("main.js".to_owned())],
                        content: "".to_owned(),
                    }
                    .into(),
                ]
                .into(),
            ),
        }
    }
}

impl PageComponent for HtmlHead {
    fn render(self, _: &str) -> HtmlElement {
        self.as_head().into()
    }
}
