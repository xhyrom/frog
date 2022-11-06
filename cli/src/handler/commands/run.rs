use std::{
    cell::RefCell,
    fs,
    io::{Error, ErrorKind, Result},
    rc::Rc,
};

use clap::ArgMatches;
use frog_lang::{lexer::Lexer, parser::Parser};
use frog_lang_evaluator::{builtin_functions::new_builtins, env::Env, object::Object, Evaluator};
use frog_logger::error;

pub fn handle(matches: &ArgMatches) {
    let file = matches.get_one::<String>("file").unwrap();

    let file_content = fs::read_to_string(file);
    if file_content.is_err() {
        error!("File not found");
        return;
    }

    let file_content = file_content.unwrap();

    let mut evaluator = get_evaluator(file);
    let runner = run_line(&mut evaluator, file_content);

    if runner.is_err() {
        error!("{}", runner.err().unwrap());
    }
}

fn get_evaluator(file: &String) -> Evaluator {
    let builtins = new_builtins();

    let env = Env::from(builtins);
    let evaluator = Evaluator::new(Rc::new(RefCell::new(env)), file.to_owned());

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
