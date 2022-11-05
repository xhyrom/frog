use clap::ArgMatches;
use frog_core::FrogCore;
use frog_logger::{debug, error, info};

pub fn handle(matches: &ArgMatches) -> () {
    let directory = matches.get_one::<String>("directory").unwrap();

    println!();

    debug!("Directory: {}", directory);

    let success = FrogCore::init(directory);
    if success {
        info!("Successfully created project");
    } else {
        error!("Failed to create project");
    }
}
