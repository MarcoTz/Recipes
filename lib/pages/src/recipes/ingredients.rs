use crate::PageComponent;
use html::{
    attribute::Attribute,
    elements::{Div, HeaderSize, Headline, HtmlElement, Li, Ul},
};
use recipes::{Ingredient, IngredientSection};
use std::rc::Rc;

fn render_ingredient(ing: Ingredient, date_format: &str) -> Li {
    Li {
        attributes: vec![],
        content: Rc::new(
            vec![
                ing.measure.render(date_format),
                ing.ingredient.to_string().into(),
            ]
            .into(),
        ),
    }
    .into()
}

impl PageComponent for IngredientSection {
    fn render(self, date_format: &str) -> HtmlElement {
        let ingredient_list = Ul {
            attributes: vec![],
            items: self
                .ingredients
                .into_iter()
                .map(|it| render_ingredient(it, date_format))
                .collect(),
        };
        if self.header.is_empty() {
            ingredient_list.into()
        } else {
            Div {
                attributes: vec![
                    Attribute::Id(self.header.clone()),
                    Attribute::Class(vec!["level2".to_owned()]),
                ],
                content: Rc::new(
                    vec![
                        Headline {
                            attributes: vec![],
                            size: HeaderSize::H3,
                            content: Rc::new(self.header.into()),
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
}
