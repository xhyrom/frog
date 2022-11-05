use std::collections::HashMap;

use lang::evaluator::object::Object;

use crate::config;

pub fn new_builtins() -> HashMap<String, Object> {
    let mut builtins = HashMap::new();
    builtins.insert(
        String::from("register_workspace"),
        Object::Builtin(1, register_workspace),
    );
    builtins.insert(
        String::from("bash"),
        Object::Builtin(1, bash),
    );
    builtins
}

fn register_workspace(args: Vec<Object>) -> Object {
    match &args[0] {
        Object::String(ref s) => {
            config::register_workspace(s.to_string());

            Object::Null
        }
        _ => Object::Error(String::from(
            "Argument to `register_workspace` must be a string",
        )),
    }
}

fn bash(args: Vec<Object>) -> Object {
    match &args[0] {
        Object::String(s) => {
            let output = std::process::Command::new("bash")
                .arg("-c")
                .arg(s)
                .output()
                .expect("failed to execute process");

            if output.status.success() {
                return Object::String(String::from_utf8(output.stdout).unwrap());
            } else {
                return Object::Error(String::from_utf8(output.stderr).unwrap());
            }
        }
        o => return Object::Error(format!("argument to `bash` not supported, got {}", o)),
    }
}