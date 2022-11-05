use std::{
    fs,
    io::{Error, ErrorKind},
    path::Path,
    sync::{Mutex, MutexGuard},
};

use lazy_static::lazy_static;

lazy_static! {
    static ref Workspaces: Mutex<Vec<String>> = Mutex::new(Vec::new());
}

pub fn register_workspace(name: String) {
    if Workspaces.lock().unwrap().contains(&name) {
        return;
    };

    Workspaces.lock().unwrap().push(name);
}

pub fn get_workspaces() -> MutexGuard<'static, Vec<std::string::String>> {
    Workspaces.lock().unwrap()
}

pub fn find(path: String) -> std::io::Result<String> {
    let mut previous = String::new();
    let mut current = path;

    while current != previous {
        for entry in fs::read_dir(&current)? {
            let entry = entry?;
            let path = entry.path();

            if path.to_str().unwrap().to_string().ends_with(".frog") {
                let content = fs::read_to_string(path)?;
                drop(previous);
                drop(current);

                return Ok(content);
            }
        }

        previous = current.clone();

        if let Some(parent) = Path::new(&current).parent() {
            current = parent.to_str().unwrap().to_string();
        } else {
            break;
        }
    }

    drop(previous);
    drop(current);

    Err(Error::new(ErrorKind::NotFound, "uwu.frog not found"))
}
