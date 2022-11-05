use std::{fs, path::Path};

pub mod config;
pub mod eval;

pub struct FrogCore;

impl FrogCore {
    pub fn init(directory: &String) -> bool {
        if directory != "." {
            match fs::create_dir_all(directory) {
                Ok(_) => (),
                Err(_) => return false,
            };
        }

        let path = Path::new(directory);

        let code = String::from(format!(
            "fun build() {{
    print(\"Building...\");
}}"));

        match fs::write(path.join("uwu.frog"), code) {
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
