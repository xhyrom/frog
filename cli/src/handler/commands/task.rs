use std::{collections::HashMap, env, io::Result};

use clap::{ArgMatches};
use frog_core::{eval, config};
use frog_logger::{error, log, colors};

fn run_task(config_path: String, task: String, args: &Vec<String>) -> Result<()> {
    let run = eval::run(config_path, HashMap::new());
    if run.is_err() {
        return Err(run.err().unwrap());
    }

    let run = run.unwrap();


    eval::run_task(
        run,
        task.to_owned(), 
        args.to_owned(), 
    )
}

pub fn handle(matches: &ArgMatches, fallback: bool) -> () {
    #[allow(unused_assignments)]
    let mut task = "".to_string();
    let mut args = Vec::new();

    if fallback {
        task = matches.subcommand_name().unwrap().to_string();
    } else {
        task = matches.get_one::<String>("task").unwrap().to_string();

        if matches.try_contains_id("args").unwrap() {
            args = matches.get_many::<String>("args").unwrap().map(|x| x.to_string()).collect::<Vec<String>>();
        }
    }

    log!(
        format!("{}[{}]{} info", colors::GRAY, "main", colors::GREEN).as_str(),
        colors::RED,
        "Running task {}", task
    );

    let config_path = env::current_dir();
    if config_path.is_err() {
        error!("Failed to get current directory");
        return;
    }

    let config_path = &config_path.unwrap().to_str().unwrap().to_string();

    let main_run = run_task(config_path.to_string(), task.to_string(), &args);
    if main_run.is_err() {
        log!(
            format!("{}[{}]{} error", colors::GRAY, "main", colors::RED).as_str(),
            colors::RED,
            "{}", main_run.err().unwrap()
        )
    }

    log!(
        format!("{}[{}]{} info", colors::GRAY, "main", colors::GREEN).as_str(),
        colors::RED,
        "Task {} completed", task
    );

    for workspace in config::get_workspaces().iter() {
        let mut path = config_path.clone();
        path.push_str(format!("/{}", workspace.as_str()).as_str());

        log!(
            format!("{}[{}]{} info", colors::GRAY, workspace, colors::GREEN).as_str(),
            colors::RED,
            "Running task {}", task
        );

        let x_task = run_task(path, task.to_string(), &args);
        if x_task.is_err() {
            log!(
                format!("{}[{}]{} error", colors::GRAY, workspace, colors::RED).as_str(),
                colors::RED,
                "{}", x_task.err().unwrap()
            );
            continue;
        }

        log!(
            format!("{}[{}]{} info", colors::GRAY, workspace, colors::GREEN).as_str(),
            colors::RED,
            "Task {} completed", task
        );
    }

    drop(task);
}