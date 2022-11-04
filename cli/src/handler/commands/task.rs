use std::{collections::HashMap, env};

use clap::ArgMatches;
use frog_core::{eval};
use frog_logger::{error, info};

pub fn handle(matches: &ArgMatches, fallback: bool) -> () {
    #[allow(unused_assignments)]
    let mut task = "".to_string();
    let mut args = Vec::new();

    if fallback {
        task = matches.subcommand_name().unwrap().to_string();
    } else {
        task = matches.get_one::<String>("task").unwrap().to_string();

        if matches.try_contains_id("args").is_ok() {
            args = matches.get_many::<String>("args").unwrap().map(|x| x.to_string()).collect::<Vec<String>>();
        }
    }

    info!("Running task {}", task);

    let config_path = env::current_dir();
    if config_path.is_err() {
        error!("Failed to get current directory");
        return;
    }

    let config_path = config_path.unwrap().to_str().unwrap().to_string();

    let run = eval::run_task(
        task.to_owned(), 
        config_path, 
        args, 
        HashMap::new()
    );
    if run.is_err() {
        error!("{}", run.err().unwrap());
        return;
    }

    info!("Task {} completed", task);

    drop(task);
}