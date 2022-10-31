use clap::{ArgMatches};
use frog_core::FrogCore;
use frog_logger::{debug, info, error};

use crate::parser::args::Language;

pub fn handle(matches: &ArgMatches) -> () {
    let name = matches.get_one::<String>("name").unwrap();
    let directory = matches.get_one::<String>("directory").unwrap();
    let language = matches.get_one::<Language>("language").unwrap();

    println!();

    debug!("Name: {}", name);
    debug!("Language: {}", language);
    debug!("Directory: {}", directory);

    println!();

    let success = FrogCore::init(name.to_owned(), directory, language.to_string().to_owned());
    if success {
        info!("Successfully created project");
    } else {
        error!("Failed to create project");
    }
}