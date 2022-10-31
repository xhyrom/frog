use clap::ArgMatches;
use frog_core::{FrogCore};
use frog_logger::{info, error};

fn show_logs(command: &String) -> bool {
    return !command.starts_with("@");
}

pub fn handle(matches: &ArgMatches, fallback: bool) -> () {
    #[allow(unused_assignments)]
    let mut task = "".to_string();
    if fallback {
        task = matches.subcommand_name().unwrap().to_string();
    } else {
        task = matches.get_one::<String>("task").unwrap().to_string();
    }

    let config = FrogCore::find_config();
    if config.is_none() {
        error!("No config file found");
        return;
    }

    let config = config.unwrap();

    for t in config.tasks {
        if t.name == task {
            for command in t.commands {
                let command = &command;

                if show_logs(command) {
                    info!("Running command: {}", command);
                }

                let output = std::process::Command::new("sh")
                    .arg("-c")
                    .arg(
                        if !show_logs(command) { command[1..command.len()].to_string() } else { command.to_string() }
                    )
                    .output()
                    .expect("Failed to execute command");
                
                println!("{}", String::from_utf8_lossy(&output.stdout));
            }
            break;
        }
    }
}