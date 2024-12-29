use super::{footer::Footer, header::Header, html_head::HtmlHead, Page, PageComponent};
use chrono::Local;
use html::{
    attribute::Attribute,
    elements::{Body, HeaderSize, Headline},
    html_document::HtmlDocument,
};
use recipes::{Recipe, Tag};
use std::rc::Rc;

pub mod recipe_list;
use recipe_list::RecipeList;

pub struct TagDetails {
    pub tag: Tag,
    pub recipes: RecipeList,
    pub num_recipes: usize,
}

impl TagDetails {
    pub fn new(tag: Tag, recipes: &[&Recipe]) -> TagDetails {
        TagDetails {
            tag,
            num_recipes: recipes.len(),
            recipes: RecipeList::new(recipes),
        }
    }
}

impl Page for TagDetails {
    fn render(self, date_format: &str) -> HtmlDocument {
        HtmlDocument {
            head: HtmlHead {
                title: self.tag.to_string(),
                relative_up: true,
            }
            .as_head(),
            body: Body {
                attributes: vec![Attribute::Id("body_tag".to_owned())],
                content: Rc::new(
                    vec![
                        Header {
                            index_link: "../index.html".to_owned(),
                            tag_link: "../tag_overview.html".to_owned(),
                        }
                        .render(date_format),
                        Headline {
                            attributes: vec![],
                            size: HeaderSize::H1,
                            content: Rc::new(self.tag.render(date_format)),
                        }
                        .into(),
                        self.recipes.render(date_format),
                        Footer {
                            created_date: Local::now().date_naive(),
                            num_recipes: self.num_recipes,
                        }
                        .render(date_format),
                    ]
                    .into(),
                ),
            },
        }
    }
}
