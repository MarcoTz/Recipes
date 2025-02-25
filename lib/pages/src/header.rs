use super::{PageComponent, RenderParameters};
use html::{
    attribute::Attribute,
    elements::{Div, HtmlElement, Script, A},
};
use std::rc::Rc;

pub struct Header;

const JS: &str = "
var recipes = [%s];
function random_recipe(){
    var ind = Math.floor(Math.random()*recipes.length);
    let recipe = recipes[ind];
    window.location.href = recipe;
}
";

impl PageComponent for Header {
    fn render(self, params: &mut RenderParameters) -> HtmlElement {
        Div {
            attributes: vec![Attribute::Id("header".to_owned())],
            content: Rc::new(
                vec![
                    Script {
                        attributes: vec![Attribute::Style("display:none;".to_owned())],
                        content: JS
                            .replace(
                                "%s",
                                &params
                                    .all_recipes
                                    .iter()
                                    .map(|rec| {
                                        format!(
                                            "\"{}recipes/{}.html\"",
                                            params.get_link_prefix(),
                                            rec.replace(" ", "")
                                        )
                                    })
                                    .collect::<Vec<String>>()
                                    .join(", "),
                            )
                            .into(),
                    }
                    .into(),
                    Div {
                        attributes: vec![Attribute::Id("headline".to_owned())],
                        content: Rc::new(
                            A {
                                attributes: vec![Attribute::Href(
                                    params.get_link_prefix() + "index.html",
                                )],
                                content: Rc::new("All Recipes".to_owned().into()),
                            }
                            .into(),
                        ),
                    }
                    .into(),
                    Div {
                        attributes: vec![],
                        content: Rc::new(
                            A {
                                attributes: vec![
                                    Attribute::Href("#".to_owned()),
                                    Attribute::OnClick("random_recipe()".to_owned()),
                                ],
                                content: Rc::new("Random Recipe".to_owned().into()),
                            }
                            .into(),
                        ),
                    }
                    .into(),
                    Div {
                        attributes: vec![Attribute::Id("tagslink".to_owned())],
                        content: Rc::new(
                            A {
                                attributes: vec![Attribute::Href(
                                    params.get_link_prefix() + "tag_overview.html",
                                )],
                                content: Rc::new("Tags".to_owned().into()),
                            }
                            .into(),
                        ),
                    }
                    .into(),
                ]
                .into(),
            ),
        }
        .into()
    }
}
