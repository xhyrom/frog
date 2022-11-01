use std::fmt;
use clap::{arg, Command, value_parser};

#[derive(clap::ValueEnum, Clone)]
pub enum Language {
    C,
    Cpp,
    Crystal,
    Go,
    Rust,
    TypeScript,
    Other,
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match *self {
           Language::C => write!(f, "C"),
           Language::Cpp => write!(f, "Cpp"),
           Language::Crystal => write!(f, "Crystal"),
           Language::Go => write!(f, "go"),
           Language::Rust => write!(f, "Rust"),
           Language::TypeScript => write!(f, "TypeScript"),
           Language::Other => write!(f, "Other"),
       }
    }
}

pub fn command() -> Command {
    Command::new("frog")
        .about("A frog that greets you")
        .long_about("A frog that greets you with a name and a number of times")
        .author("xHyroM")
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .arg(arg!(-v --version))
        .subcommand(
            Command::new("init")
                .about("Initialize a new frog project")
                .arg(
                    arg!(<language> "Language to use for the project").value_parser(value_parser!(Language)).ignore_case(true)
                )
                .args(&[
                    arg!(--name [name]).default_value("frog_project"),
                    arg!(--directory [directory]).default_value("."),
                ])
                .arg_required_else_help(true)
        )
        .subcommand(
            Command::new("task")
                .about("Run a task")
                .arg(
                    arg!(<task> "Task to run")
                )
                .arg_required_else_help(true)
        )
}

pub fn parse() -> Command {
    command()
}