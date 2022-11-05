use std::collections::HashMap;

use crate::object::Object;

pub fn new() -> HashMap<String, Object> {
    let mut builtins = HashMap::new();
    builtins.insert(String::from("sqrt"), Object::Builtin(1, frog_sqrt));
    builtins.insert(String::from("pow"), Object::Builtin(2, frog_pow));
    builtins.insert(String::from("floor"), Object::Builtin(1, frog_floor));
    builtins
}

fn frog_sqrt(args: Vec<Object>) -> Object {
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

fn frog_pow(args: Vec<Object>) -> Object {
    #![allow(unused_assignments)]
    let mut num: f64 = 0.0;
    let mut exp: f64 = 0.0;

    match &args[0] {
        Object::Float(i) => num = *i,
        Object::Int(i) => num = *i as f64,
        o => return Object::Error(format!("argument to `math_pow` not supported, got {}", o)),
    }

    match &args[1] {
        Object::Float(i) => exp = *i,
        Object::Int(i) => exp = *i as f64,
        o => return Object::Error(format!("argument to `math_pow` not supported, got {}", o)),
    }

    Object::Float(num.powf(exp))
}

fn frog_floor(args: Vec<Object>) -> Object {
    match &args[0] {
        Object::Int(i) => Object::Float(
            (*i as f64).floor()
        ),
        Object::Float(i) => Object::Float(
            (*i).floor()
        ),
        o => Object::Error(format!("argument to `math_floor` not supported, got {}", o)),
    }
}