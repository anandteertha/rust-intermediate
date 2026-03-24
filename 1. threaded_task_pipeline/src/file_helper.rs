use std::{fs, path::Path};

pub fn load_text(file_path: &str) -> Result<String, String> {
    let path = Path::new(file_path);
    let file_content = fs::read_to_string(path);
    match file_content {
        Ok(text) => Ok(text),
        Err(e) => Err(format!("error occurred while parsing file: {}", e)),
    }
}
