pub mod builtins_functions;
pub mod builtins_modules;
pub mod env;
pub mod object;

use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::rc::Rc;

use frog_lang::ast::*;
use frog_lang::lexer::Lexer;
use frog_lang::parser::Parser;

use self::env::*;
use self::object::*;

#[derive(Debug)]
pub struct Evaluator {
    env: Rc<RefCell<Env>>,
    builtin_modules: HashMap<String, HashMap<String, Object>>,
    path: PathBuf,
}

impl Evaluator {
    pub fn new(env: Rc<RefCell<Env>>, path: String) -> Self {
        Evaluator { env, builtin_modules: builtins_modules::new_builtins(), path: Path::new(&path).to_path_buf() }
    }

    fn is_truthy(obj: Object) -> bool {
        match obj {
            Object::Null | Object::Bool(false) => false,
            _ => true,
        }
    }

    fn error(msg: String) -> Object {
        Object::Error(msg)
    }

    fn is_error(obj: &Object) -> bool {
        match obj {
            Object::Error(_) => true,
            _ => false,
        }
    }

    pub fn eval(&mut self, program: Program) -> Option<Object> {
        for stmt in program {
            if stmt == Stmt::Blank {
                continue;
            }

            match self.eval_stmt(stmt) {
                Some(Object::Error(msg)) => return Some(Object::Error(msg)),
                _ => (),
            }
        }

        None
    }

    fn eval_block_stmt(&mut self, stmts: BlockStmt) -> Option<Object> {
        let mut result = None;

        for stmt in stmts {
            if stmt == Stmt::Blank {
                continue;
            }

            match self.eval_stmt(stmt) {
                Some(Object::ReturnValue(value)) => return Some(Object::ReturnValue(value)),
                Some(Object::Error(msg)) => return Some(Object::Error(msg)),
                obj => result = obj,
            }
        }

        result
    }

    fn eval_stmt(&mut self, stmt: Stmt) -> Option<Object> {
        match stmt {
            Stmt::Let(ident, expr) => {
                let value = match self.eval_expr(expr) {
                    Some(value) => value,
                    None => return None,
                };
                if Self::is_error(&value) {
                    Some(value)
                } else {
                    let Ident(name) = ident;
                    self.env.borrow_mut().set(name, &value);
                    None
                }
            }
            Stmt::Import(expr) => self.eval_import(expr),
            Stmt::Expr(expr) => self.eval_expr(expr),
            Stmt::Return(expr) => {
                let value = match self.eval_expr(expr) {
                    Some(value) => value,
                    None => return None,
                };
                if Self::is_error(&value) {
                    Some(value)
                } else {
                    Some(Object::ReturnValue(Box::new(value)))
                }
            }
            _ => None,
        }
    }

    fn eval_expr(&mut self, expr: Expr) -> Option<Object> {
        match expr {
            Expr::Ident(ident) => Some(self.eval_ident(ident)),
            Expr::Literal(literal) => Some(self.eval_literal(literal)),
            Expr::Prefix(prefix, right_expr) => {
                if let Some(right) = self.eval_expr(*right_expr) {
                    Some(self.eval_prefix_expr(prefix, right))
                } else {
                    None
                }
            }
            Expr::Infix(infix, left_expr, right_expr) => {
                let left = self.eval_expr(*left_expr);
                let right = self.eval_expr(*right_expr);

                if left.is_some() && right.is_some() {
                    Some(self.eval_infix_expr(infix, left.unwrap(), right.unwrap()))
                } else {
                    None
                }
            }
            Expr::Index(left_expr, index_expr) => {
                let left = self.eval_expr(*left_expr);
                let index = self.eval_expr(*index_expr);
                if left.is_some() && index.is_some() {
                    Some(self.eval_index_expr(left.unwrap(), index.unwrap()))
                } else {
                    None
                }
            }
            Expr::If {
                cond,
                consequence,
                alternative,
            } => self.eval_if_expr(*cond, consequence, alternative),
            Expr::Func { name, params, body } => Some(self.eval_func_expr(name, params, body)),
            Expr::Call { func, args } => Some(self.eval_call_expr(func, args)),
        }
    }

    fn eval_import(&mut self, expr: Expr) -> Option<Object> {
        let value = match self.eval_expr(expr) {
            Some(value) => value,
            None => return None,
        };
        
        match value {
            Object::String(mut path) => {
                if path.starts_with("frog::") {
                    return self.eval_import_builtin(path);
                }

                if let Some(parent) = self.path.parent() {
                    path = parent.join(path).to_str().unwrap().to_owned();
                }

                path.push_str(".frog");

                if self.path.to_str().unwrap().eq(&path) {
                    return Some(Self::error(format!("Circular imports are not allowed, tried to import {}", path)));
                };

                let file = fs::read_to_string(&path);
                if file.is_err() {
                    return Some(Self::error(format!("Failed to import {}", path)));
                }
            
                let file = file.unwrap();

                let mut parser = Parser::new(Lexer::new(&file));
                let program = parser.parse();
                let errors = parser.get_errors();
            
                if errors.len() > 0 {
                    for err in errors {
                        return Some(Self::error(format!("{}", err)));
                    }
                };

                if let Some(evaluated) = self.eval(program) {
                    match evaluated {
                        Object::Error(err) => return Some(Self::error(format!("{}", err))),
                        _ => println!("{}", evaluated),
                    }
                }
            }
            _ => return Some(Self::error(format!("{} is not a string", value))),
        };

        None
    }

    fn eval_import_builtin(&mut self, path: String) -> Option<Object> {
        let methods = self.builtin_modules.get(&path);
        if methods.is_none() {
            return Some(Object::Error(format!("{} is not a builtin module", path)));
        }

        let methods = methods.unwrap();

        for method in methods {
            self.env.borrow_mut().set(
                format!("{}_{}", path.replace("frog::", ""), method.0),
                method.1
            );
        }

        None
    }

    fn eval_ident(&mut self, ident: Ident) -> Object {
        let Ident(name) = ident;

        match self.env.borrow_mut().get(name.clone()) {
            Some(value) => value,
            None => Object::Error(String::from(format!("identifier not found: {}", name))),
        }
    }

    fn eval_prefix_expr(&mut self, prefix: Prefix, right: Object) -> Object {
        match prefix {
            Prefix::Not => self.eval_not_op_expr(right),
            Prefix::Minus => self.eval_minus_prefix_op_expr(right),
            Prefix::Plus => self.eval_plus_prefix_op_expr(right),
        }
    }

    fn eval_not_op_expr(&mut self, right: Object) -> Object {
        match right {
            Object::Bool(true) => Object::Bool(false),
            Object::Bool(false) => Object::Bool(true),
            Object::Null => Object::Bool(true),
            _ => Object::Bool(false),
        }
    }

    fn eval_minus_prefix_op_expr(&mut self, right: Object) -> Object {
        match right {
            Object::ReturnValue(value) => {
                self.eval_minus_prefix_op_expr(*value)
            },
            Object::Int(value) => Object::Int(-value),
            Object::Float(value) => Object::Float(-value),
            _ => Self::error(format!("unknown operator: -{}", right)),
        }
    }

    fn eval_plus_prefix_op_expr(&mut self, right: Object) -> Object {
        match right {
            Object::ReturnValue(value) => {
                self.eval_plus_prefix_op_expr(*value)
            },
            Object::Int(value) => Object::Int(value),
            Object::Float(value) => Object::Float(value),
            _ => Self::error(format!("unknown operator: {}", right)),
        }
    }

    fn eval_infix_expr(&mut self, infix: Infix, left: Object, right: Object) -> Object {
        match left {
            Object::ReturnValue(left_value) => {
                if let Object::ReturnValue(right_value) = right {
                    self.eval_infix_expr(infix, *left_value, *right_value)
                } else {
                    self.eval_infix_expr(infix, *left_value, right)
                }
            },
            Object::Int(left_value) => {
                if let Object::Int(right_value) = right {
                    self.eval_infix_int_expr(infix, left_value, right_value)
                } else {
                    Self::error(format!("type mismatch: {} {} {}", left, infix, right))
                }
            }
            Object::Float(left_value) => {
                if let Object::Float(right_value) = right {
                    self.eval_infix_float_expr(infix, left_value, right_value)
                } else {
                    Self::error(format!("type mismatch: {} {} {}", left, infix, right))
                }
            }
            Object::String(left_value) => {
                if let Object::String(right_value) = right {
                    self.eval_infix_string_expr(infix, left_value, right_value)
                } else {
                    Self::error(format!("type mismatch: {} {} {}", left_value, infix, right))
                }
            }
            _ => Self::error(format!("unknown operator: {} {} {}", left, infix, right)),
        }
    }

    fn eval_index_expr(&mut self, left: Object, index: Object) -> Object {
        match left {
            Object::Array(ref array) => {
                if let Object::Int(i) = index {
                    self.eval_array_index_expr(array.clone(), i)
                } else {
                    Self::error(format!("index operator not supported: {}", left))
                }
            }
            Object::Hash(ref hash) => match index {
                Object::Int(_) | Object::Bool(_) | Object::String(_) => match hash.get(&index) {
                    Some(o) => o.clone(),
                    None => Object::Null,
                },
                Object::Error(_) => index,
                _ => Self::error(format!("unusable as hash key: {}", index)),
            },
            _ => Self::error(format!("uknown operator: {} {}", left, index)),
        }
    }

    fn eval_array_index_expr(&mut self, array: Vec<Object>, index: i64) -> Object {
        let max = array.len() as i64;

        if index < 0 || index > max {
            return Object::Null;
        }

        match array.get(index as usize) {
            Some(o) => o.clone(),
            None => Object::Null,
        }
    }

    fn eval_infix_int_expr(&mut self, infix: Infix, left: i64, right: i64) -> Object {
        match infix {
            Infix::Plus => Object::Int(left + right),
            Infix::Minus => Object::Int(left - right),
            Infix::Multiply => Object::Int(left * right),
            Infix::Divide => Object::Int(left / right),
            Infix::LessThan => Object::Bool(left < right),
            Infix::LessThanEqual => Object::Bool(left <= right),
            Infix::GreaterThan => Object::Bool(left > right),
            Infix::GreaterThanEqual => Object::Bool(left >= right),
            Infix::Equal => Object::Bool(left == right),
            Infix::NotEqual => Object::Bool(left != right),
        }
    }

    fn eval_infix_float_expr(&mut self, infix: Infix, left: f64, right: f64) -> Object {
        match infix {
            Infix::Plus => Object::Float(left + right),
            Infix::Minus => Object::Float(left - right),
            Infix::Multiply => Object::Float(left * right),
            Infix::Divide => Object::Float(left / right),
            Infix::LessThan => Object::Bool(left < right),
            Infix::LessThanEqual => Object::Bool(left <= right),
            Infix::GreaterThan => Object::Bool(left > right),
            Infix::GreaterThanEqual => Object::Bool(left >= right),
            Infix::Equal => Object::Bool(left == right),
            Infix::NotEqual => Object::Bool(left != right),
        }
    }

    fn eval_infix_string_expr(&mut self, infix: Infix, left: String, right: String) -> Object {
        match infix {
            Infix::Plus => Object::String(format!("{}{}", left, right)),
            _ => Object::Error(String::from(format!(
                "unknown operator: {} {} {}",
                left, infix, right
            ))),
        }
    }

    fn eval_literal(&mut self, literal: Literal) -> Object {
        match literal {
            Literal::Int(value) => Object::Int(value),
            Literal::Float(value) => Object::Float(value),
            Literal::Bool(value) => Object::Bool(value),
            Literal::String(value) => Object::String(value),
            Literal::Char(value) => Object::Char(value),
            Literal::Array(objects) => self.eval_array_literal(objects),
            Literal::Hash(pairs) => self.eval_hash_literal(pairs),
        }
    }

    fn eval_array_literal(&mut self, objects: Vec<Expr>) -> Object {
        Object::Array(
            objects
                .iter()
                .map(|e| self.eval_expr(e.clone()).unwrap_or(Object::Null))
                .collect::<Vec<_>>(),
        )
    }

    fn eval_hash_literal(&mut self, pairs: Vec<(Expr, Expr)>) -> Object {
        let mut hash = HashMap::new();

        for (key_expr, value_expr) in pairs {
            let key = self.eval_expr(key_expr).unwrap_or(Object::Null);
            if Self::is_error(&key) {
                return key;
            }

            let value = self.eval_expr(value_expr).unwrap_or(Object::Null);
            if Self::is_error(&value) {
                return value;
            }

            hash.insert(key, value);
        }

        Object::Hash(hash)
    }

    fn eval_if_expr(
        &mut self,
        cond: Expr,
        consequence: BlockStmt,
        alternative: Option<BlockStmt>,
    ) -> Option<Object> {
        let cond = match self.eval_expr(cond) {
            Some(cond) => cond,
            None => return None,
        };

        if Self::is_error(&cond) {
            Some(cond)
        } else if Self::is_truthy(cond) {
            self.eval_block_stmt(consequence)
        } else if let Some(alt) = alternative {
            self.eval_block_stmt(alt)
        } else {
            None
        }
    }

    fn eval_func_expr(&mut self, name: Ident, params: Vec<Ident>, body: BlockStmt) -> Object {
        let Ident(name) = name;
        let func = Object::Func(name.to_owned(), params, body, Rc::clone(&self.env));
        self.env.borrow_mut().set(name, &func);

        func
    }

    fn eval_call_expr(&mut self, func: Box<Expr>, args: Vec<Expr>) -> Object {
        let args = args
            .iter()
            .map(|e| self.eval_expr(e.clone()).unwrap_or(Object::Null))
            .collect::<Vec<_>>();

        let (params, body, env) = match self.eval_expr(*func) {
            Some(Object::Func(_, params, body, env)) => (params, body, env),
            Some(Object::Builtin(expect_param_num, f)) => {
                if expect_param_num < 0 || expect_param_num == args.len() as i32 {
                    return f(args);
                } else {
                    return Self::error(format!(
                        "wrong number of arguments. got={}, want={}",
                        args.len(),
                        expect_param_num,
                    ));
                }
            }
            Some(o) => return Self::error(format!("{} is not valid function", o)),
            None => return Object::Null,
        };

        if params.len() != args.len() {
            return Self::error(format!(
                "wrong number of arguments: {} expected but {} given",
                params.len(),
                args.len()
            ));
        }

        let current_env = Rc::clone(&self.env);
        let mut scoped_env = Env::new_with_outer(Rc::clone(&env));
        let list = params.iter().zip(args.iter());
        for (_, (ident, o)) in list.enumerate() {
            let Ident(name) = ident.clone();
            scoped_env.set(name, o);
        }

        self.env = Rc::new(RefCell::new(scoped_env));

        let object = self.eval_block_stmt(body);

        self.env = current_env;

        match object {
            Some(o) => o,
            None => Object::Null,
        }
    }
}
