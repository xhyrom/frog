use clap::Command;

mod commands;

pub fn handle(command: Command) -> () {
    let matches = command.get_matches();
    match matches.subcommand() {
        Some(("init", matches)) => commands::init::handle(matches),
        Some(("task", matches)) => commands::task::handle(matches, false),
        _ => commands::task::handle(&matches, true),
    }
}