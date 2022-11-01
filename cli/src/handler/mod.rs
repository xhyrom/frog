use clap::Command;

mod commands;

pub fn handle(command: Command) -> () {
    let matches = command.get_matches();
    match matches.subcommand() {
        Some(("init", matches)) => commands::init::handle(matches),
        Some(("task", matches)) => commands::task::handle(matches, false),
        Some(("version", _)) => commands::version::handle(),
        _ => {
            if matches.subcommand_name().is_some() {
                commands::task::handle(&matches, true);
            }

            if matches.contains_id("version") {
                commands::version::handle();
            }
        },
    }
}