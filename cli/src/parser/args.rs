use clap::{arg, Command};

pub fn command() -> Command {
    Command::new("frog")
        .about("A frog that greets you")
        .long_about("A frog that greets you with a name and a number of times")
        .author("xHyroM")
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .arg(arg!(-v - -version))
        .subcommand(
            Command::new("run")
                .about("Run a file")
                .arg(arg!(<file> "File to run"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("compile")
                .about("Compile a file")
                .arg(arg!(<file> "File to compile"))
                .arg_required_else_help(true),
        )
}

pub fn parse() -> Command {
    command()
}
