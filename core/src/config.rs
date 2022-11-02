use std::{fs, io::{Error, ErrorKind}};

pub fn find(path: String) -> std::io::Result<String> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        let path_string = path.to_str().unwrap().to_string();

        if path_string.contains("uwu.frog") {
            let content = fs::read_to_string(path)?;
            drop(path_string);

            return Ok(content);
        }
    }

    Err(Error::new(ErrorKind::NotFound, "uwu.frog not found"))
}