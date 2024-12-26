use parse_markdown::{load_markdown, recipe::parse_recipe};
use std::path::PathBuf;

static RECIPE_PATH: &str = "./Recipes";
static IMG_PATH: &str = "./html/img";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let recipe_strs = load_markdown(PathBuf::from(RECIPE_PATH)).map_err(Box::new)?;
    for recipe_str in recipe_strs {
        let recipe = parse_recipe(recipe_str.contents, PathBuf::from(IMG_PATH))?;
        println!("{:?}", recipe.images);
    }

    Ok(())
}
