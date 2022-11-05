use clap::Command;

mod commands;

pub fn handle(command: Command) -> () {
    let matches = command.get_matches();
    match matches.subcommand() {
        Some(("run", matches)) => commands::run::handle(matches),
        Some(("compile", matches)) => commands::compile::handle(matches),
        Some(("version", _)) => commands::version::handle(),
        _ => {
            if matches.try_contains_id("version").unwrap() {
                commands::version::handle();
            }
        }
    }
}
