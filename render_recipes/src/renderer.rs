use html::render::Render;
use pages::{Index, Page, RecipeDetails, TagDetails, TagOverview};
use parse_markdown::{load_markdown, recipe::parse_recipe};
use recipes::Recipe;
use std::{
    collections::HashMap,
    fs::{create_dir_all, File},
    io::prelude::Write,
    path::PathBuf,
};

pub struct AllPages {
    index: Index,
    tag_overview: TagOverview,
    recipe_details: Vec<RecipeDetails>,
    tag_details: Vec<TagDetails>,
}

pub struct PageHtmls {
    index: String,
    tag_overview: String,
    recipe_details: Vec<(String, String)>,
    tag_details: Vec<(String, String)>,
}

pub fn load_recipes(
    recipe_path: PathBuf,
    img_path: PathBuf,
) -> Result<Vec<Recipe>, Box<dyn std::error::Error>> {
    let recipe_strs = load_markdown(recipe_path).map_err(Box::new)?;
    let mut recipes = vec![];
    for recipe_str in recipe_strs {
        let recipe = parse_recipe(recipe_str.contents, img_path.clone())?;
        recipes.push(recipe)
    }
    Ok(recipes)
}

pub fn create_pages(recipes: Vec<Recipe>) -> Result<AllPages, Box<dyn std::error::Error>> {
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

    Ok(AllPages {
        index,
        tag_overview,
        recipe_details: recipe_pages,
        tag_details: tag_pages,
    })
}

pub fn render_pages(
    pages: AllPages,
    date_format: &str,
) -> Result<PageHtmls, Box<dyn std::error::Error>> {
    let index_str = pages.index.render(date_format).render();
    let tag_overview_str = pages.tag_overview.render(date_format).render();

    let mut page_strs = vec![];
    for recipe_page in pages.recipe_details {
        let name = recipe_page.recipe_name.clone();
        let recipe_page_str = recipe_page.render(date_format).render();
        page_strs.push((name, recipe_page_str));
    }
    let mut tag_strs = vec![];
    for tag_page in pages.tag_details {
        let name = tag_page.name.clone();
        let tag_str = tag_page.render(date_format).render();
        tag_strs.push((name, tag_str));
    }

    Ok(PageHtmls {
        index: index_str,
        tag_overview: tag_overview_str,
        recipe_details: page_strs,
        tag_details: tag_strs,
    })
}

pub fn write_pages(pages: PageHtmls, out_dir: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let index_path = out_dir.join("index.html");
    let mut index_file = File::create(index_path)?;
    index_file.write_all(pages.index.as_bytes())?;

    let tag_overview_path = out_dir.join("tag_overview.html");
    let mut tags_file = File::create(tag_overview_path)?;
    tags_file.write_all(pages.tag_overview.as_bytes())?;

    let recipe_dir = out_dir.join("recipes");
    create_dir_all(recipe_dir.clone())?;
    for (recipe_name, recipe_str) in pages.recipe_details {
        let mut recipe_path = recipe_dir.join(recipe_name.replace(" ", ""));
        recipe_path.set_extension("html");
        let mut recipe_file = File::create(recipe_path)?;
        recipe_file.write_all(recipe_str.as_bytes())?;
    }

    let tags_dir = out_dir.join("tags");
    create_dir_all(tags_dir.clone())?;
    for (tag_name, tag_str) in pages.tag_details {
        let mut tag_path = tags_dir.join(tag_name.replace(" ", ""));
        tag_path.set_extension("html");
        let mut tag_file = File::create(tag_path)?;
        tag_file.write_all(tag_str.as_bytes())?;
    }

    Ok(())
}
