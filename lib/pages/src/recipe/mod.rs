use crate::{footer::Footer, header::Header, html_head::HtmlHead, Page, PageComponent};
use chrono::Local;
use html::{attribute::Attribute, elements::Body, html_document::HtmlDocument};
use std::rc::Rc;

pub mod recipe_contents;
pub mod recipe_images;

use recipe_contents::RecipeContents;
use recipe_images::RecipeImages;

pub struct RecipeDetails {
    pub recipe_name: String,
    pub num_recipes: usize,
    pub recipe_contents: RecipeContents,
    pub recipe_images: RecipeImages,
}

impl Page for RecipeDetails {
    fn render(self, date_format: &str) -> HtmlDocument {
        HtmlDocument {
            head: HtmlHead {
                title: self.recipe_name,
            }
            .as_head(),
            body: Body {
                attributes: vec![Attribute::Id("body_recipe".to_owned())],
                content: Rc::new(
                    vec![
                        Header {
                            index_link: "../index.html".to_owned(),
                            tag_link: "../tag_overview.html".to_owned(),
                        }
                        .render(date_format),
                        self.recipe_contents.render(date_format),
                        self.recipe_images.render(date_format),
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
