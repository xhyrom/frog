use std::{fs, path::Path, collections::HashMap};

use config::Config;

mod config;

pub struct FrogCore {
}

impl FrogCore {
    pub fn test() -> () {
        println!("Hello from FrogCore!");
        println!("{:?}", config::find(".".to_string()).unwrap());
    }

    pub fn find_config() -> Option<Config> {
        let file = config::find(".".to_string());
        if file.is_err() {
            return None;
        }

        return Some(file.unwrap());
    }

    pub fn init(name: String, directory: &String, language: String) -> bool {
        if directory != "." {
            match fs::create_dir_all(directory) {
                Ok(_) => (),
                Err(_) => return false,
            };
        }

        let mut map = HashMap::new();
        let path = Path::new(directory);

        map.insert("name".to_string(), name);
        map.insert("language".to_string(), language);

        let config = config::Config {
            variables: map,
            tasks: vec![config::Task {
                name: "build".to_string(),
                commands: vec!["echo \"Hello from Frog!\"".to_string(), "echo \"${name} - ${language}\"".to_string()],
            }],
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
