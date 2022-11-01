use std::{collections::HashMap, io::{ErrorKind, Error}};

use lang::compiler::{Program, Instruction};

use crate::config::{Config, Task};

pub fn into_config(pgrm: Program) -> Config {
    let mut variables = HashMap::new();
    let mut functions = HashMap::new();

    let mut variable_now = false;
    let mut call_now = false;

    let mut variable_name = String::new();
    let mut function_name = String::new();
    let mut call_name = String::new();

    let mut variable_value = String::new();
    let mut function_value: Vec<Task> = Vec::new();
    let mut call_value = Vec::new();

    for i in pgrm.instructions {
        match i {
            Instruction::String(s) => {
                if variable_now {
                    variable_value = s;
                }
                else if call_now {
                    call_value.push(s);
                }
            }
            Instruction::Number(n) => {
                variable_value = n.to_string();
                call_value.push(n.to_string());
            }
            Instruction::Variable(v) => {
                variable_now = true;
                variable_name = v;
            }
            Instruction::Function(f, _) => {
                function_name = f;
            }
            Instruction::Call(c, _) => {
                call_now = true;
                call_name = c;
            },
            Instruction::DeclareEnd => {
                variable_now = false;

                variables.insert(variable_name.to_owned(), variable_value.to_owned());
            },
            Instruction::CallEnd => {
                call_now = false;

                function_value.push(Task {
                    name: call_name.to_owned(),
                    commands: call_value.to_owned(),
                });
                call_value = Vec::new();
            },
            Instruction::FunctionEnd => {
                functions.insert(function_name.to_owned(), function_value);
                function_value = Vec::new();
            }
        }
    }

    drop(variable_now);
    drop(call_now);

    drop(variable_name);
    drop(function_name);
    drop(call_name);

    drop(variable_value);
    drop(function_value);
    drop(call_value);

    return Config {
        variables,
        functions,
    };
}

pub fn run_task(config: &Config, task: String) -> std::io::Result<()> {
    let task = config.functions.get(&task).unwrap();

    for t in task {
        match t.name.as_str() {
            "echo" => {
                for c in &t.commands {
                    println!("{}", c);
                }
            }
            "bash" => {
                for c in &t.commands {
                    let command = std::process::Command::new("bash")
                        .envs(&config.variables)
                        .arg("-c")
                        .arg(c)
                        .spawn();

                    if command.is_err() {
                        return Err(Error::new(ErrorKind::Other, "Failed to execute command"));
                    }

                    let command = command.unwrap();
                    let output = command
                        .wait_with_output();

                    if output.is_err() {
                        return Err(Error::new(ErrorKind::Other, "Failed to execute command"));
                    }

                    let output = output.unwrap();

                    if !output.status.success() {
                        return Err(
                            Error::new(
                                ErrorKind::Other,
                                format!(
                                    "Failed to execute command -\nStderr: {}\nStdout: {}",
                                    String::from_utf8_lossy(&output.stderr),
                                    String::from_utf8_lossy(&output.stdout)
                                )
                            )
                        );
                    }
                }
            }
            _ => {
                let func = config.functions.get(&t.name);
                if func.is_some() {
                    let task = run_task(config, t.name.to_owned());

                    if task.is_err() {
                        return Err(task.err().unwrap());
                    }

                    continue;
                }

                return Err(Error::new(ErrorKind::Other, format!("Task {} not found", t.name)));
            }
        }
    }

    return Ok(());
}