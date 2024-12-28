pub mod recipe_list;
pub mod recipe_search;
pub mod recipe_sort;

use super::{footer::Footer, header::Header, html_head::HtmlHead, Page, PageComponent};
use chrono::Local;
use html::{
    attribute::Attribute,
    elements::{Body, Div, HeaderSize, Headline},
    html_document::HtmlDocument,
};
use recipes::Recipe;
use std::rc::Rc;

use recipe_list::RecipeList;
use recipe_search::RecipeSearch;
use recipe_sort::RecipeSort;

pub struct Index {
    pub recipes: RecipeList,
    pub sort: RecipeSort,
    pub search: RecipeSearch,
}

impl Index {
    pub fn new(recipes: Vec<Recipe>) -> Index {
        Index {
            recipes: RecipeList::new(recipes),
            sort: RecipeSort,
            search: RecipeSearch,
        }
    }
}

impl Page for Index {
    fn render(self, date_format: &str) -> HtmlDocument {
        let num_recipes = self.recipes.recipes.len();
        HtmlDocument {
            head: HtmlHead {
                title: "Recipes".to_owned(),
            }
            .as_head(),
            body: Body {
                attributes: vec![Attribute::Id("body_index".to_owned())],
                content: Rc::new(
                    vec![
                        Header {
                            index_link: "index.html".to_owned(),
                            tag_link: "tag_overview.html".to_owned(),
                        }
                        .render(date_format),
                        Headline {
                            attributes: vec![],
                            size: HeaderSize::H1,
                            content: Rc::new("Recipes".to_owned().into()),
                        }
                        .into(),
                        Div {
                            attributes: vec![Attribute::Id("recipe_list_container".to_owned())],
                            content: Rc::new(
                                vec![
                                    self.sort.render(date_format),
                                    self.search.render(date_format),
                                    Div {
                                        attributes: vec![Attribute::Class(
                                            vec!["space".to_owned()],
                                        )],
                                        content: Rc::new("".to_owned().into()),
                                    }
                                    .into(),
                                    Div {
                                        attributes: vec![Attribute::Class(
                                            vec!["hline".to_owned()],
                                        )],
                                        content: Rc::new("".to_owned().into()),
                                    }
                                    .into(),
                                    self.recipes.render(date_format),
                                ]
                                .into(),
                            ),
                        }
                        .into(),
                        Footer {
                            created_date: Local::now().date_naive(),
                            num_recipes,
                        }
                        .render(date_format),
                    ]
                    .into(),
                ),
            },
        }
    }
}
