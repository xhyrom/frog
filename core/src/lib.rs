use std::{fs, path::Path, collections::HashMap};

use config::{Config, Task};

//use crate::syntax::{lexer, eval, parser};

pub mod config;
pub mod eval;

pub struct FrogCore;

impl FrogCore {
    pub fn init(name: String, directory: &String, language: String) -> bool {
        if directory != "." {
            match fs::create_dir_all(directory) {
                Ok(_) => (),
                Err(_) => return false,
            };
        }

        let mut variables_map = HashMap::new();
        let path = Path::new(directory);

        variables_map.insert("name".to_string(), name);
        variables_map.insert("language".to_string(), language);

        let mut functions_map = HashMap::new();

        functions_map.insert(
            "init".to_string(),
            vec![
                Task {
                    name: "bash".to_string(),
                    commands: vec!["echo Hello from Frog!".to_string()],
                },
            ],
        );

        let config = Config {
            variables: variables_map,
            functions: functions_map,
        };
        
        match fs::write(path.join("uwu.frog"), config.serialize()) {
            Ok(_) => return true,
            Err(_) => return false,
        };
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
