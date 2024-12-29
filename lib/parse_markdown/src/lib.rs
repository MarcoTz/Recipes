pub mod errors;
pub mod images;
pub mod ingredient;
pub mod parse_steps;
pub mod recipe;
pub mod steps;
pub mod tag;

use errors::Error;
use std::{
    ffi::OsString,
    fs::{read_dir, read_to_string},
    path::PathBuf,
    str::FromStr,
};

pub struct RecipeSource {
    pub file_name: PathBuf,
    pub contents: String,
}

pub fn load_markdown(path: PathBuf) -> Result<Vec<RecipeSource>, Error> {
    let dir_contents = read_dir(path)?;
    let mut sources = vec![];
    for m_entry in dir_contents {
        let entry = m_entry?;
        let path = entry.path();
        if path.extension() == Some(&OsString::from_str("md").unwrap()) {
            let contents = read_to_string(entry.path())?;
            sources.push(RecipeSource {
                file_name: path,
                contents,
            });
        }
    }
    Ok(sources)
}
