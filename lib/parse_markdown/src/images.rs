use super::errors::Error;
use std::path::PathBuf;

pub fn load_recipe_images(recipe_name: &str, images_dir: PathBuf) -> Result<Vec<String>, Error> {
    let recipe_name = recipe_name.replace(" ", "");
    let dir_contents = std::fs::read_dir(images_dir)?;
    let mut images = vec![];
    for dir_entry in dir_contents {
        let dir_entry = dir_entry?;
        let path = dir_entry.path();
        let file_name = path
            .file_name()
            .ok_or(Error::IO(std::io::ErrorKind::Other))?;
        let file_name = file_name
            .to_str()
            .ok_or(Error::IO(std::io::ErrorKind::Other))?;
        if file_name.contains(&recipe_name) {
            images.push(file_name.to_owned());
        }
    }
    Ok(images)
}
