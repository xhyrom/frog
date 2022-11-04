use std::collections::HashMap;

use lang::evaluator::object::Object;

use crate::config;

pub fn new_builtins() -> HashMap<String, Object> {
    let mut builtins = HashMap::new();
    builtins.insert(
        String::from("register_workspace"),
        Object::Builtin(1, register_workspace),
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
