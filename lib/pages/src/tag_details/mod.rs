use super::{
    footer::Footer, header::Header, html_head::HtmlHead, Page, PageComponent, RenderParameters,
};
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
    pub fn new(tag: Tag, recipes: &[&Recipe], num_recipes: usize) -> TagDetails {
        TagDetails {
            tag,
            num_recipes,
            recipes: RecipeList::new(recipes),
        }
    }
}

impl Page for TagDetails {
    fn render(self, params: &mut RenderParameters) -> HtmlDocument {
        params.depth = 1;
        HtmlDocument {
            head: HtmlHead {
                title: self.tag.to_string(),
            }
            .as_head(params),
            body: Body {
                attributes: vec![Attribute::Id("body_tag".to_owned())],
                content: Rc::new(
                    vec![
                        Header.render(params),
                        Headline {
                            attributes: vec![],
                            size: HeaderSize::H1,
                            content: Rc::new(self.tag.render(params)),
                        }
                        .into(),
                        self.recipes.render(params),
                        Footer {
                            created_date: Local::now().date_naive(),
                            num_recipes: self.num_recipes,
                        }
                        .render(params),
                    ]
                    .into(),
                ),
            },
        }
    }
}
