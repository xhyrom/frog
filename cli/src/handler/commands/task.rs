use std::collections::HashMap;

use clap::ArgMatches;
use frog_core::{eval};
use frog_logger::{error, info};

pub fn handle(matches: &ArgMatches, fallback: bool) -> () {
    #[allow(unused_assignments)]
    let mut task = "".to_string();

    if fallback {
        task = matches.subcommand_name().unwrap().to_string();
    } else {
        task = matches.get_one::<String>("task").unwrap().to_string();
    }

    info!("Running task {}", task);

    let run = eval::run_task(task.to_owned(), ".".to_string(), HashMap::new());
    if run.is_err() {
        error!("{}", run.err().unwrap());
        return;
    }

    info!("Task {} completed", task);

    drop(task);
}