use std::path::PathBuf;

static RECIPE_PATH: &str = "./Recipes";
static IMG_PATH: &str = "./html/img";
static DATE_FORMAT: &str = "%d.%m.%Y";
static OUT_PATH: &str = "html";

mod renderer;
use renderer::{create_pages, load_recipes, render_pages, write_pages};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let recipes = load_recipes(PathBuf::from(RECIPE_PATH), PathBuf::from(IMG_PATH))?;
    let pages = create_pages(recipes)?;
    let html_contents = render_pages(pages, DATE_FORMAT)?;
    write_pages(html_contents, PathBuf::from(OUT_PATH))?;

    Ok(())
}
