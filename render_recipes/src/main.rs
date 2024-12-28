use html::render::Render;
use pages::{Index, Page, RecipeDetails, TagDetails, TagOverview};
use parse_markdown::{load_markdown, recipe::parse_recipe};
use std::{
    collections::HashMap,
    fs::{create_dir_all, File},
    io::prelude::Write,
    path::PathBuf,
};

static RECIPE_PATH: &str = "./Recipes";
static IMG_PATH: &str = "./html/img";
static DATE_FORMAT: &str = "%d.%m.%Y";
static OUT_PATH: &str = "html";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let recipe_strs = load_markdown(PathBuf::from(RECIPE_PATH)).map_err(Box::new)?;
    let mut recipes = vec![];
    for recipe_str in recipe_strs {
        let recipe = parse_recipe(recipe_str.contents, PathBuf::from(IMG_PATH))?;
        recipes.push(recipe)
    }

    let index = Index::new(recipes.clone());
    let tag_overview = TagOverview::new(&recipes);
    let mut tags = HashMap::new();
    let mut recipe_pages = vec![];
    for recipe in recipes.iter() {
        let recipe_page = RecipeDetails::new(recipe, recipes.len());
        recipe_pages.push(recipe_page);
        for recipe_tag in recipe.tags.iter() {
            match tags.get_mut(recipe_tag) {
                None => {
                    tags.insert(recipe_tag.clone(), vec![recipe]);
                }
                Some(recipes) => {
                    recipes.push(recipe);
                }
            }
        }
    }

    let mut tag_pages = vec![];
    for (tag, recipes) in tags.into_iter() {
        let tag_page = TagDetails::new(tag, recipes.as_slice());
        tag_pages.push(tag_page);
    }

    let index_str = index.render(DATE_FORMAT).render();
    let tag_overview_str = tag_overview.render(DATE_FORMAT).render();

    let mut page_strs = vec![];
    for recipe_page in recipe_pages {
        let name = recipe_page.recipe_name.clone();
        let recipe_page_str = recipe_page.render(DATE_FORMAT).render();
        page_strs.push((name, recipe_page_str));
    }
    let mut tag_strs = vec![];
    for tag_page in tag_pages {
        let name = tag_page.name.clone();
        let tag_str = tag_page.render(DATE_FORMAT).render();
        tag_strs.push((name, tag_str));
    }

    let index_path = PathBuf::from(OUT_PATH).join("index.html");
    let mut index_file = File::create(index_path)?;
    index_file.write_all(index_str.as_bytes())?;

    let tag_overview_path = PathBuf::from(OUT_PATH).join("tag_overview.html");
    let mut tags_file = File::create(tag_overview_path)?;
    tags_file.write_all(tag_overview_str.as_bytes())?;

    let recipe_dir = PathBuf::from(OUT_PATH).join("recipes");
    create_dir_all(recipe_dir.clone())?;
    for (recipe_name, recipe_str) in page_strs {
        let mut recipe_path = recipe_dir.join(recipe_name.replace(" ", ""));
        recipe_path.set_extension("html");
        let mut recipe_file = File::create(recipe_path)?;
        recipe_file.write_all(recipe_str.as_bytes())?;
    }

    let tags_dir = PathBuf::from(OUT_PATH).join("tags");
    create_dir_all(tags_dir.clone())?;
    for (tag_name, tag_str) in tag_strs {
        let mut tag_path = tags_dir.join(tag_name.replace(" ", ""));
        tag_path.set_extension("html");
        let mut tag_file = File::create(tag_path)?;
        tag_file.write_all(tag_str.as_bytes())?;
    }

    Ok(())
}
