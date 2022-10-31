use clap::ArgMatches;
use frog_logger::{info};

pub fn handle(matches: &ArgMatches, fallback: bool) -> () {
    #[allow(unused_assignments)]
    let mut task = "".to_string();
    if fallback {
        task = matches.subcommand_name().unwrap().to_string();
    } else {
        task = matches.get_one::<String>("task").unwrap().to_string();
    }

    // TASSSSSSSK
    info!("Task name {}", task); 
}