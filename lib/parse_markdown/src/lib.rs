pub mod errors;
pub mod ingredient;
pub mod parse_steps;
pub mod recipe;

use errors::Error;
use std::{
    ffi::OsString,
    fs::{read_dir, read_to_string},
    path::PathBuf,
    str::FromStr,
};

pub fn load_markdown(path: PathBuf) -> Result<Vec<String>, Error> {
    let dir_contents = read_dir(path)?;
    let mut contents = vec![];
    for m_entry in dir_contents {
        let entry = m_entry?;
        let path = entry.path();
        if path.extension() == Some(&OsString::from_str("md").unwrap()) {
            contents.push(read_to_string(entry.path())?);
        }
    }
    Ok(contents)
}
