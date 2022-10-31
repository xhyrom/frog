use clap::{arg, Command};

pub fn command() -> Command {
    Command::new("frog")
        .about("A frog that greets you")
        .long_about("A frog that greets you with a name and a number of times")
        .author("xHyroM")
        //.arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("init")
                .about("Initialize a new frog project")
                .arg(
                    arg!(<language> "Language to use for the project")
                )
                .arg_required_else_help(true)
        )
}

pub fn parse() -> Command {
    command()
}