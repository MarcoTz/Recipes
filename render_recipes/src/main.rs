use recipes::{Measurement, Recipe, Unit};

fn main() {
    let test_recipe = Recipe {
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
    println!("{test_recipe}")
}
