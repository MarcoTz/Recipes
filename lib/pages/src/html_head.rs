use super::PageComponent;
use html::{
    attribute::Attribute,
    elements::{Head, HtmlElement, Link, Script},
};
use std::rc::Rc;

pub struct HtmlHead {
    pub title: String,
    pub relative_up: bool,
}

impl HtmlHead {
    pub fn as_head(self) -> Head {
        let prefix = if self.relative_up { "../" } else { "" };
        let js_src = format!("{prefix}main.js");
        let css_src = format!("{prefix}main.css");
        Head {
            title: self.title,
            content: Rc::new(
                vec![
                    Link {
                        attributes: vec![
                            Attribute::Rel("stylesheet".to_owned()),
                            Attribute::Href(css_src),
                        ],
                    }
                    .into(),
                    Script {
                        attributes: vec![Attribute::Src(js_src)],
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
