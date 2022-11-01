use clap::ArgMatches;
use frog_core::{config, syntax};
use frog_logger::{error, info};

pub fn handle(matches: &ArgMatches, fallback: bool) -> () {
    #[allow(unused_assignments)]
    let mut task = "".to_string();

    if fallback {
        task = matches.subcommand_name().unwrap().to_string();
    } else {
        task = matches.get_one::<String>("task").unwrap().to_string();
    }

    let config = config::find(".".to_string());
    if config.is_err() {
        error!("No config file found");
        return;
    }

    let config = config::get_config(config.unwrap());
    if config.is_err() {
        error!("{}", config.err().unwrap());
        return;
    }

    let config = config.unwrap();
    
    info!("Running task: {}", task);

    match syntax::eval::run_task(&config, task.to_owned()) {
        Ok(_) => info!("Task {} completed", task),
        Err(e) => error!("{}", e),
    }

    drop(task);
}