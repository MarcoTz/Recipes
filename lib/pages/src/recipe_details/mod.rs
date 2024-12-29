use crate::{
    footer::Footer, header::Header, html_head::HtmlHead, Page, PageComponent, RenderParameters,
};
use chrono::Local;
use html::{attribute::Attribute, elements::Body, html_document::HtmlDocument};
use recipes::Recipe;
use std::rc::Rc;

pub mod recipe_contents;
pub mod recipe_images;

use recipe_contents::RecipeContents;
use recipe_images::RecipeImages;

#[derive(PartialEq)]
pub struct RecipeDetails {
    pub recipe_name: String,
    pub num_recipes: usize,
    pub recipe_contents: RecipeContents,
    pub recipe_images: RecipeImages,
}

impl RecipeDetails {
    pub fn new(recipe: &Recipe, num_recipes: usize) -> RecipeDetails {
        let images = RecipeImages::new(recipe.image_filenames.clone());
        RecipeDetails {
            recipe_name: recipe.name.clone(),
            num_recipes,
            recipe_contents: RecipeContents::new(recipe),
            recipe_images: images,
        }
    }
}

impl Page for RecipeDetails {
    fn render(self, params: &mut RenderParameters) -> HtmlDocument {
        params.depth = 1;
        HtmlDocument {
            head: HtmlHead {
                title: self.recipe_name,
            }
            .as_head(params),
            body: Body {
                attributes: vec![Attribute::Id("body_recipe".to_owned())],
                content: Rc::new(
                    vec![
                        Header.render(params),
                        self.recipe_contents.render(params),
                        self.recipe_images.render(params),
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
