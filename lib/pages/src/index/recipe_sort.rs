use crate::PageComponent;
use html::{
    attribute::Attribute,
    elements::{Div, HtmlElement, Select, SelectOption},
};
use std::rc::Rc;

pub struct RecipeSort;

impl PageComponent for RecipeSort {
    fn render(self, _: &str) -> HtmlElement {
        Div {
            attributes: vec![Attribute::Id("recipe_list_sort".to_owned())],
            content: Rc::new(
                vec![
                    "Sort by".to_owned().into(),
                    Select {
                        attributes: vec![
                            Attribute::Id("sort_key".to_owned()),
                            Attribute::OnChange("sort()".to_owned()),
                        ],
                        options: vec![
                            SelectOption {
                                value: "name_desc".to_owned(),
                                content: Rc::new("Name (desc.)".to_owned().into()),
                            },
                            SelectOption {
                                value: "name_asc".to_owned(),
                                content: Rc::new("Name (asc.)".to_owned().into()),
                            },
                        ],
                    }
                    .into(),
                ]
                .into(),
            ),
        }
        .into()
    }
}
