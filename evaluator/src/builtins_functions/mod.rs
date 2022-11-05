use std::collections::HashMap;

use super::object::Object;

pub fn new_builtins() -> HashMap<String, Object> {
    let mut builtins = HashMap::new();
    builtins.insert(String::from("len"), Object::Builtin(1, frog_len));
    builtins.insert(String::from("print"), Object::Builtin(-1, frog_print));
    builtins.insert(String::from("typeof"), Object::Builtin(1, frog_typeof));
    builtins
}

fn frog_len(args: Vec<Object>) -> Object {
    match &args[0] {
        Object::String(s) => Object::Int(s.len() as i64),
        Object::Array(o) => Object::Int(o.len() as i64),
        o => Object::Error(format!("argument to `len` not supported, got {}", o)),
    }
}

fn frog_print(args: Vec<Object>) -> Object {
    println!(
        "{}",
        args.iter()
            .map(|x| format!("{}", x))
            .collect::<Vec<String>>()
            .join(" ")
    );

    Object::Null
}

fn frog_typeof(args: Vec<Object>) -> Object {
    match &args[0] {
        Object::Int(_) => Object::String("int".to_string()),
        Object::Float(_) => Object::String("float".to_string()),
        Object::String(_) => Object::String("string".to_string()),
        Object::Char(_) => Object::String("char".to_string()),
        Object::Bool(_) => Object::String("bool".to_string()),
        Object::Array(_) => Object::String("array".to_string()),
        Object::Hash(_) => Object::String("hash".to_string()),
        Object::Func(_, _, _, _) => Object::String("function".to_string()),
        Object::Builtin(_, _) => Object::String("builtin".to_string()),
        Object::Null => Object::String("null".to_string()),
        Object::ReturnValue(_) => Object::String("return".to_string()),
        Object::Error(_) => Object::String("error".to_string()),
    }
}