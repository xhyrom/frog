use std::collections::HashMap;

use crate::object::Object;

pub fn new() -> HashMap<String, Object> {
    let mut builtins = HashMap::new();
    builtins.insert(String::from("math_sqrt"), Object::Builtin(1, frog_math_sqrt));
    builtins.insert(String::from("math_pow"), Object::Builtin(2, frog_math_pow));
    builtins.insert(String::from("math_floor"), Object::Builtin(1, frog_math_floor));
    builtins
}

fn frog_math_sqrt(args: Vec<Object>) -> Object {
    match &args[0] {
        Object::Int(i) => Object::Float(
            (*i as f64).sqrt()
        ),
        Object::Float(i) => Object::Float(
            (*i).sqrt()
        ),
        o => Object::Error(format!("argument to `math_sqrt` not supported, got {}", o)),
    }
}

fn frog_math_pow(args: Vec<Object>) -> Object {
    let mut num: f64 = 0.0;
    let mut exp: f64 = 0.0;

    match &args[0] {
        Object::Float(i) => num = *i,
        Object::Int(i) => num = *i as f64,
        _ => (),
    }

    match &args[1] {
        Object::Float(i) => exp = *i,
        Object::Int(i) => exp = *i as f64,
        _ => (),
    }

    Object::Float(num.powf(exp))
}

fn frog_math_floor(args: Vec<Object>) -> Object {
    match &args[0] {
        Object::Int(i) => Object::Float(
            (*i as f64).floor()
        ),
        Object::Float(i) => Object::Float(
            (*i).floor()
        ),
        o => Object::Error(format!("argument to `math_sqrt` not supported, got {}", o)),
    }
}