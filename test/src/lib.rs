#[cfg(test)]
mod recipe_tests {

    use parse_markdown::{load_markdown, parse_recipe, RecipeSource};
    use std::fmt;
    use std::path::PathBuf;

    static RECIPE_PATH: &str = "../Recipes";
    static IMG_PATH: &str = "../html/img";

    enum TestResult {
        Fail(String),
        Success,
    }

    impl TestResult {
        fn failed(&self) -> bool {
            matches!(self, TestResult::Fail(_))
        }
    }

    impl fmt::Display for TestResult {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                TestResult::Fail(msg) => write!(f, ".... failed: \n {msg}"),
                TestResult::Success => f.write_str(".... ok"),
            }
        }
    }

    struct ParseResult {
        source_file: PathBuf,
        parsed: TestResult,
        reparsed: TestResult,
        reparsed_same: TestResult,
    }

    fn test_recipe(source: RecipeSource) -> ParseResult {
        let parsed = parse_recipe(source.contents.clone(), PathBuf::from(IMG_PATH));
        let recipe = match parsed {
            Err(err) => {
                return ParseResult {
                    source_file: source.file_name,
                    parsed: TestResult::Fail(err.to_string()),
                    reparsed: TestResult::Fail("Parsing Failed".to_owned()),
                    reparsed_same: TestResult::Fail("Parsing Failed".to_owned()),
                }
            }
            Ok(recipe) => recipe,
        };

        let printed = format!("{}", recipe);

        let reparsed = parse_recipe(printed.clone(), PathBuf::from(IMG_PATH));
        let recipe_new = match reparsed {
            Err(err) => {
                return ParseResult {
                    source_file: source.file_name,
                    parsed: TestResult::Success,
                    reparsed: TestResult::Fail(err.to_string()),
                    reparsed_same: TestResult::Fail("Reparsing failed".to_owned()),
                }
            }
            Ok(rec) => rec,
        };

        ParseResult {
            source_file: source.file_name,
            parsed: TestResult::Success,
            reparsed: TestResult::Success,
            reparsed_same: if recipe == recipe_new {
                TestResult::Success
            } else {
                TestResult::Fail(format!(
                    "Parsed and reparsed differ\nparsed:\n{:?}\n\nreparsed:\n{:?}",
                    recipe, recipe_new
                ))
            },
        }
    }
    #[test]
    fn run_tests() -> Result<(), String> {
        let recipe_strs =
            load_markdown(PathBuf::from(RECIPE_PATH)).map_err(|err| err.to_string())?;

        let mut results = vec![];
        for source in recipe_strs {
            let result = test_recipe(source);
            results.push(result);
        }

        let mut num_tests = 0;
        let mut fail_parse = 0;
        let mut fail_reparse = 0;
        let mut fail_reparse_same = 0;

        for result in results {
            println!("Tests for {:?}", result.source_file);
            num_tests += 1;
            println!("\t Parsing {}", result.parsed);
            if result.parsed.failed() {
                fail_parse += 1
            }
            println!("\t Reparsing {}", result.reparsed);
            if result.reparsed.failed() {
                fail_reparse += 1
            }
            println!("\t Reparsed is same {}", result.reparsed_same);
            if result.reparsed_same.failed() {
                fail_reparse_same += 1
            }
        }

        println!("");
        println!("Ran {num_tests} tests");
        println!("parsing fails: {fail_parse}");
        println!("reparsing fails: {fail_reparse}");
        println!("reparsing different from parsed: {fail_reparse_same}");

        assert_eq!(fail_parse, 0);
        assert_eq!(fail_reparse, 0);
        assert_eq!(fail_reparse_same, 0);
        Ok(())
    }
}
