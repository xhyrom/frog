use clap::Command;

mod commands;

pub fn handle(command: Command) -> () {
    match command.get_matches().subcommand() {
        Some(("init", matches)) => commands::init::handle(matches),
        _ => unreachable!()
    }
}