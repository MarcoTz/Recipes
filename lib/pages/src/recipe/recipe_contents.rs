use crate::PageComponent;
use html::{
    attribute::Attribute,
    elements::{Div, HeaderSize, Headline, HtmlElement, Li, Ol, Ul, A},
};
use recipes::{IngredientSection, Recipe, StepSection};
use std::rc::Rc;

#[derive(PartialEq)]
pub struct RecipeContents {
    pub recipe: Recipe,
}

impl RecipeContents {
    pub fn new(recipe: &Recipe) -> RecipeContents {
        RecipeContents {
            recipe: recipe.clone(),
        }
    }
}

impl PageComponent for RecipeContents {
    fn render(self, _: &str) -> HtmlElement {
        let mut tags_rendered: Vec<HtmlElement> = self
            .recipe
            .get_tag_urls("../tags")
            .into_iter()
            .map(|(tag, tag_url)| {
                A {
                    attributes: vec![Attribute::Href(tag_url)],
                    content: Rc::new(tag.into()),
                }
                .into()
            })
            .collect();

        tags_rendered.insert(
            0,
            Headline {
                size: HeaderSize::H2,
                attributes: vec![],
                content: Rc::new("Tags".to_owned().into()),
            }
            .into(),
        );
        let mut ingredients_rendered: Vec<HtmlElement> = self
            .recipe
            .ingredients
            .into_iter()
            .map(render_ingredient_section)
            .collect();
        ingredients_rendered.insert(
            0,
            Headline {
                attributes: vec![],
                size: HeaderSize::H2,
                content: Rc::new("Ingredients".to_owned().into()),
            }
            .into(),
        );

        let mut steps_rendered: Vec<HtmlElement> = self
            .recipe
            .steps
            .into_iter()
            .map(render_step_section)
            .collect();
        steps_rendered.insert(
            0,
            Headline {
                size: HeaderSize::H2,
                attributes: vec![],
                content: Rc::new("Steps".to_owned().into()),
            }
            .into(),
        );

        Div {
            attributes: vec![
                Attribute::Id(self.recipe.name.clone()),
                Attribute::Class(vec!["level1".to_owned()]),
            ],
            content: Rc::new(
                vec![
                    Headline {
                        attributes: vec![],
                        size: HeaderSize::H1,
                        content: Rc::new(self.recipe.name.into()),
                    }
                    .into(),
                    Div {
                        attributes: vec![
                            Attribute::Id("ingredients".to_owned()),
                            Attribute::Class(vec!["level2".to_owned()]),
                        ],
                        content: Rc::new(ingredients_rendered.into()),
                    }
                    .into(),
                    Div {
                        attributes: vec![
                            Attribute::Id("steps".to_owned()),
                            Attribute::Class(vec!["level2".to_owned()]),
                        ],
                        content: Rc::new(steps_rendered.into()),
                    }
                    .into(),
                    Div {
                        attributes: vec![
                            Attribute::Id("tags".to_owned()),
                            Attribute::Class(vec!["level2".to_owned()]),
                        ],
                        content: Rc::new(tags_rendered.into()),
                    }
                    .into(),
                ]
                .into(),
            ),
        }
        .into()
    }
}

fn render_ingredient_section(sec: IngredientSection) -> HtmlElement {
    let ingredient_list = Ul {
        attributes: vec![],
        items: sec
            .ingredients
            .into_iter()
            .map(|it| Li {
                attributes: vec![],
                content: Rc::new(format!("{it}").into()),
            })
            .collect(),
    };
    if sec.header.is_empty() {
        ingredient_list.into()
    } else {
        Div {
            attributes: vec![
                Attribute::Id(sec.header.clone()),
                Attribute::Class(vec!["level2".to_owned()]),
            ],
            content: Rc::new(
                vec![
                    Headline {
                        attributes: vec![],
                        size: HeaderSize::H3,
                        content: Rc::new(sec.header.into()),
                    }
                    .into(),
                    ingredient_list.into(),
                ]
                .into(),
            ),
        }
        .into()
    }
}

fn render_step_section(sec: StepSection) -> HtmlElement {
    let step_ol = Ol {
        attributes: vec![],
        items: sec
            .steps
            .into_iter()
            .map(|item| Li {
                attributes: vec![],
                content: Rc::new(item.into()),
            })
            .collect(),
    };
    if sec.header.is_empty() {
        step_ol.into()
    } else {
        Div {
            attributes: vec![
                Attribute::Id(sec.header.clone()),
                Attribute::Class(vec!["level2".to_owned()]),
            ],
            content: Rc::new(
                vec![
                    Headline {
                        attributes: vec![],
                        size: HeaderSize::H3,
                        content: Rc::new(sec.header.into()),
                    }
                    .into(),
                    step_ol.into(),
                ]
                .into(),
            ),
        }
        .into()
    }
}
