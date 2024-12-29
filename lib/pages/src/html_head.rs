use super::{PageComponent, RenderParameters};
use html::{
    attribute::Attribute,
    elements::{Head, HtmlElement, Link, Script},
};
use std::rc::Rc;

pub struct HtmlHead {
    pub title: String,
}

impl HtmlHead {
    pub fn as_head(self, params: &mut RenderParameters) -> Head {
        let prefix = params.get_link_prefix();
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
    fn render(self, params: &mut RenderParameters) -> HtmlElement {
        self.as_head(params).into()
    }
}
