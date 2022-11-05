use std::{cell::RefCell, rc::Rc, io::{Result, ErrorKind, Error}, fs};

use clap::ArgMatches;
use frog_lang::{parser::Parser, lexer::Lexer};
use frog_lang_evaluator::{Evaluator, builtins::new_builtins, env::Env, object::Object};
use frog_logger::error;

pub fn handle(matches: &ArgMatches) {
    let file = matches.get_one::<String>("file").unwrap();

    let file = fs::read_to_string(file);
    if file.is_err() {
        error!("File not found");
        return;
    }

    let file = file.unwrap();

    let mut evaluator = get_evaluator();
    let runner = run_line(&mut evaluator, file);

    if runner.is_err() {
        error!("{}", runner.err().unwrap());
    }
}

fn get_evaluator() -> Evaluator {
    let builtins = new_builtins();

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