use std::{cell::RefCell, rc::Rc, io::{Error, ErrorKind, Result}, collections::HashMap};
use lang::{evaluator::{Evaluator, env::Env, builtins::new_builtins, object::Object}, parser::{Parser}, lexer::{Lexer}};

use crate::config;

pub mod builtins;

pub fn run(config_path: String, custom_builtins: HashMap<String, Object>) -> Result<Evaluator> {
    let mut evaluator = get_evaluator(custom_builtins);
    let config = config::find(config_path);

    if config.is_err() {
        return Err(Error::new(ErrorKind::Other, config.err().unwrap()));
    }

    let config = config.unwrap();
    let runner = run_line(&mut evaluator, config);

    if runner.is_err() {
        return Err(Error::new(ErrorKind::Other, runner.err().unwrap()));
    }
    
    Ok(evaluator)
}

pub fn run_task(mut evaluator: Evaluator, name: String, args: Vec<String>) -> Result<()> {
    let run_task = run_line(
        &mut evaluator,
        format!("{}({})", name, args.iter().map(|x| format!("\"{}\"", x)).collect::<Vec<String>>().join(", "))
    ); 

    if run_task.is_err() {
        return Err(Error::new(ErrorKind::Other, run_task.err().unwrap()));
    }

    Ok(())
}

fn get_evaluator(custom_builtins: HashMap<String, Object>) -> Evaluator {
    let mut builtins = new_builtins();
    builtins.extend(custom_builtins);
    builtins.extend(builtins::new_builtins());

    let env = Env::from(builtins);
    let evaluator = Evaluator::new(Rc::new(RefCell::new(env)));

    evaluator
}

fn run_line(evaluator: &mut Evaluator, line: String) -> Result<()> {
    let mut parser = Parser::new(Lexer::new(&line));
    let program = parser.parse();
    let errors = parser.get_errors();

    if errors.len() > 0 {
        for err in errors {
            return Err(Error::new(ErrorKind::Other, format!("{}", err)));
        }
    };

    if let Some(evaluated) = evaluator.eval(program) {
        match evaluated {
            Object::Error(err) => return Err(Error::new(ErrorKind::Other, format!("{}", err))),
            _ => println!("{}", evaluated),
        }
    }

    Ok(())
}