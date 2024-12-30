use crate::{PageComponent, RenderParameters};
use html::{
    attribute::Attribute,
    elements::{Div, HtmlElement, A},
};
use recipes::{Recipe, Tag};
use std::{collections::HashMap, rc::Rc};

pub struct TagList {
    pub tags: Vec<(Tag, usize)>,
}

impl TagList {
    pub fn new(recipes: &[Recipe]) -> TagList {
        let mut tags = HashMap::new();
        for recipe in recipes {
            for recipe_tag in recipe.tags.iter().cloned() {
                match tags.get(&recipe_tag) {
                    None => tags.insert(recipe_tag, 1),
                    Some(i) => tags.insert(recipe_tag, i + 1),
                };
            }
        }

        TagList {
            tags: tags.into_iter().collect(),
        }
    }
}

impl PageComponent for TagList {
    fn render(self, params: &mut RenderParameters) -> HtmlElement {
        let mut tag_divs = vec![];
        for (tag, num_recipes) in self.tags {
            let tag_url = tag.get_url("tags");
            let div = Div {
                attributes: vec![Attribute::Class(vec!["tag_item".to_owned()])],
                content: Rc::new(
                    vec![
                        A {
                            attributes: vec![Attribute::Href(tag_url)],
                            content: Rc::new(tag.render(params)),
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
