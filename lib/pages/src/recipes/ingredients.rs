use crate::{PageComponent, RenderParameters};
use html::{
    attribute::Attribute,
    elements::{Div, HeaderSize, Headline, HtmlElement, Li, Ul},
};
use recipes::{Ingredient, IngredientSection};
use std::rc::Rc;

fn render_ingredient(ing: Ingredient, params: &mut RenderParameters) -> Li {
    Li {
        attributes: vec![],
        content: Rc::new(vec![ing.measure.render(params), ing.ingredient.render(params)].into()),
    }
}

impl PageComponent for IngredientSection {
    fn render(self, params: &mut RenderParameters) -> HtmlElement {
        let ingredient_list = Ul {
            attributes: vec![],
            items: self
                .ingredients
                .into_iter()
                .map(|it| render_ingredient(it, params))
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
