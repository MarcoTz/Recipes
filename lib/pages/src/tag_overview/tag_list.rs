use crate::PageComponent;
use html::{
    attribute::Attribute,
    elements::{Div, HtmlElement, A},
};
use std::rc::Rc;

pub struct TagList {
    pub tags: Vec<(String, usize)>,
}

impl PageComponent for TagList {
    fn render(self, _: &str) -> HtmlElement {
        let mut tag_divs = vec![];
        for (tag, num_recipes) in self.tags {
            let mut tag_url = tag.replace(" ", "");
            tag_url.push_str(".html");
            tag_url.insert_str(0, "tags/");
            let div = Div {
                attributes: vec![Attribute::Class(vec!["tag_item".to_owned()])],
                content: Rc::new(
                    vec![
                        A {
                            attributes: vec![Attribute::Href(tag_url)],
                            content: Rc::new(tag.into()),
                        }
                        .into(),
                        format!("({num_recipes})").into(),
                    ]
                    .into(),
                ),
            }
            .into();
            tag_divs.push(div);
        }
        Div {
            attributes: vec![Attribute::Id("tags_list".to_owned())],
            content: Rc::new(tag_divs.into()),
        }
        .into()
    }
}
