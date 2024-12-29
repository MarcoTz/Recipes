use super::{footer::Footer, header::Header, html_head::HtmlHead, PageComponent, RenderParameters};
use crate::Page;
use chrono::Local;
use html::{
    attribute::Attribute,
    elements::{Body, Div, HeaderSize, Headline},
    html_document::HtmlDocument,
};
use recipes::Recipe;
use std::rc::Rc;

pub mod tag_list;
pub mod tag_search;
pub mod tag_sort;

use tag_list::TagList;
use tag_search::TagSearch;
use tag_sort::TagSort;

pub struct TagOverview {
    pub num_recipes: usize,
    pub sort: TagSort,
    pub search: TagSearch,
    pub list: TagList,
}

impl TagOverview {
    pub fn new(recipes: &[Recipe]) -> TagOverview {
        TagOverview {
            num_recipes: recipes.len(),
            sort: TagSort,
            search: TagSearch,
            list: TagList::new(recipes),
        }
    }
}

impl Page for TagOverview {
    fn render(self, params: &mut RenderParameters) -> HtmlDocument {
        params.depth = 0;
        HtmlDocument {
            head: HtmlHead {
                title: "Tags".to_owned(),
            }
            .as_head(params),
            body: Body {
                attributes: vec![
                    Attribute::Id("body_tag_overview".to_owned()),
                    Attribute::OnLoad("sort()".to_owned()),
                ],
                content: Rc::new(
                    vec![
                        Header.render(params),
                        Headline {
                            attributes: vec![],
                            size: HeaderSize::H1,
                            content: Rc::new("Tags".to_owned().into()),
                        }
                        .into(),
                        Div {
                            attributes: vec![Attribute::Id("tag_list_container".to_owned())],
                            content: Rc::new(
                                vec![
                                    self.sort.render(params),
                                    self.search.render(params),
                                    Div {
                                        attributes: vec![],
                                        content: Rc::new("".to_owned().into()),
                                    }
                                    .into(),
                                    Div {
                                        attributes: vec![],
                                        content: Rc::new("".to_owned().into()),
                                    }
                                    .into(),
                                    self.list.render(params),
                                ]
                                .into(),
                            ),
                        }
                        .into(),
                        Footer {
                            num_recipes: self.num_recipes,
                            created_date: Local::now().date_naive(),
                        }
                        .render(params),
                    ]
                    .into(),
                ),
            },
        }
    }
}
