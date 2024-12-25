use parse_markdown::recipe::parse_recipe;
use std::path::PathBuf;

static RECIPE_PATH: &str = "./Recipes";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    /*let contents = load_markdown(PathBuf::from(RECIPE_PATH)).map_err(Box::new)?;
    println!("loaded: \n{}", contents.join("\n\n"));
    Ok(())*/

    let contents = std::fs::read_to_string(PathBuf::from(RECIPE_PATH).join("VeganDampfnudeln.md"))?;
    let res = parse_recipe(contents)?;
    println!("{res}");
    Ok(())

    /*    let test_recipe = Recipe {
        name: "testing".to_owned(),
        ingredients: vec![
            (
                Measurement {
                    amount: 1.0,
                    unit: Unit::Gram,
                },
                "ingredient1".to_owned(),
            ),
            (
                Measurement {
                    amount: 2.5,
                    unit: Unit::Gram,
                },
                "ingredient2".to_owned(),
            ),
            (
                Measurement {
                    amount: 3.3,
                    unit: Unit::Liter,
                },
                "ingredient3".to_owned(),
            ),
        ],
        steps: vec![
            "do someting".to_owned(),
            "do something else".to_owned(),
            "finish".to_owned(),
        ],
        notes: vec![
            "this is for testing".to_owned(),
            "this should be formatted correctly".to_owned(),
        ],
        tags: vec!["tag1".to_owned(), "tag2".to_owned()],
    };
    println!("{test_recipe}")*/
}
