use std::{fs, io::{Error, ErrorKind}, collections::HashMap};

use crate::syntax::{eval::{into_config}, parser, lexer, compiler};

#[derive(Debug)]
pub struct Task {
    pub name: String,
    pub commands: Vec<String>,
}

#[derive(Debug)]
pub struct Config {
    pub variables: HashMap<String, String>,
    pub functions: HashMap<String, Vec<Task>>,
}

impl Config {
    pub fn serialize(self) -> String {
        let mut config = String::new();

        for variable in self.variables {
            config.push_str(&format!("declare {} = {}\n", variable.0, variable.1));
        }

        for task in self.functions {
            config.push_str(
                &format!(
                    "\ntask {} {{\n{}\n}}\n",
                    task.0,
                    task.1
                        .iter()
                        .map(|t| format!("    {}", t.commands.join("\n    ")))
                        .collect::<Vec<String>>()
                        .join("\n")
                )
            );
        }

        config
    }
}

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

pub fn get_config(content: String) -> std::io::Result<Config> {
    let raw = content.chars().collect::<Vec<char>>();

    let lex = lexer::lex(&raw);
    if lex.is_err() {
        return Err(Error::new(ErrorKind::InvalidData, lex.err().unwrap()));
    }

    let lex = lex.unwrap();

    let parse = parser::parse(&raw, lex);
    if parse.is_err() {
        return Err(Error::new(ErrorKind::InvalidData, parse.err().unwrap()));
    }

    let parse = parse.unwrap();

    let compile = compiler::compile(&raw, parse);

    Ok(into_config(compile))
}