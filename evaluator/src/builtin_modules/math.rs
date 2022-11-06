use std::collections::HashMap;

use crate::object::Object;

pub fn new() -> HashMap<String, Object> {
    let mut builtins = HashMap::new();
    builtins.insert(String::from("abs"), Object::Builtin(1, frog_abs));
    builtins.insert(String::from("sqrt"), Object::Builtin(1, frog_sqrt));
    builtins.insert(String::from("pow"), Object::Builtin(2, frog_pow));
    builtins.insert(String::from("log"), Object::Builtin(2, frog_log));
    builtins.insert(String::from("log2"), Object::Builtin(1, frog_log2));
    builtins.insert(String::from("log10"), Object::Builtin(1, frog_log10));
    builtins.insert(String::from("floor"), Object::Builtin(1, frog_floor));
    builtins.insert(String::from("ceil"), Object::Builtin(1, frog_ceil));
    builtins
}

fn frog_abs(args: Vec<Object>) -> Object {
    match &args[0] {
        Object::Int(i) => Object::Int(i.abs()),
        Object::Float(i) => Object::Float(i.abs()),
        o => Object::Error(format!("argument to `math_abs` not supported, got {}", o)),
    }
}

fn frog_sqrt(args: Vec<Object>) -> Object {
    match &args[0] {
        Object::Int(i) => Object::Float((*i as f64).sqrt()),
        Object::Float(i) => Object::Float((*i).sqrt()),
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

fn frog_log(args: Vec<Object>) -> Object {
    match &args[..] {
        [a, b] => match (a, b) {
            (Object::Int(a), Object::Int(b)) => Object::Float((*a as f64).log(*b as f64)),
            (Object::Float(a), Object::Float(b)) => Object::Float(a.log(*b)),
            (Object::Int(a), Object::Float(b)) => Object::Float((*a as f64).log(*b)),
            (Object::Float(a), Object::Int(b)) => Object::Float(a.log(*b as f64)),
            (a, b) => Object::Error(format!("argument to `math_log` not supported, got {} and {}", a, b)),
        },
        _ => Object::Error(String::from("argument to `math_log` not supported")),
    }
}

fn frog_log2(args: Vec<Object>) -> Object {
    match &args[0] {
        Object::Int(i) => Object::Float((*i as f64).log2()),
        Object::Float(i) => Object::Float((*i).log2()),
        o => Object::Error(format!("argument to `math_log2` not supported, got {}", o)),
    }
}

fn frog_log10(args: Vec<Object>) -> Object {
    match &args[0] {
        Object::Int(i) => Object::Float((*i as f64).log10()),
        Object::Float(i) => Object::Float((*i).log10()),
        o => Object::Error(format!("argument to `math_log10` not supported, got {}", o)),
    }
}

fn frog_floor(args: Vec<Object>) -> Object {
    match &args[0] {
        Object::Int(i) => Object::Float((*i as f64).floor()),
        Object::Float(i) => Object::Float(i.floor()),
        o => Object::Error(format!("argument to `math_floor` not supported, got {}", o)),
    }
}

fn frog_ceil(args: Vec<Object>) -> Object {
    match &args[0] {
        Object::Int(i) => Object::Float((*i as f64).ceil()),
        Object::Float(i) => Object::Float(i.ceil()),
        o => Object::Error(format!("argument to `math_ceil` not supported, got {}", o)),
    }
}