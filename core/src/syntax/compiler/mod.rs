/**
 * Base is from https://github.com/eatonphil/lust
 * Under Apache 2.0 license
 */

use crate::syntax::parser::*;

#[derive(Debug)]
pub enum Instruction {
    String(String),
    Number(i32),
    Variable(String),
    Function(String, usize),
    Call(String, usize),
    DeclareEnd,
    FunctionEnd,
    CallEnd,
}

#[derive(Debug)]
pub struct Program {
    pub instructions: Vec<Instruction>,
}

fn compile_function_call(
    pgrm: &mut Program,
    raw: &[char],
    fc: FunctionCall,
) {
    let len = fc.arguments.len();
    pgrm.instructions.push(Instruction::Call(fc.name.value, len));

    for arg in fc.arguments {
        compile_expression(pgrm, raw, arg);
    }

    pgrm.instructions.push(Instruction::CallEnd);
}

fn compile_literal(
    pgrm: &mut Program,
    _: &[char],
    lit: Literal,
) {
    match lit {
        Literal::Number(t) => {
            let v = t.value.parse::<i32>().unwrap();
            pgrm.instructions.push(Instruction::Number(v));
        }
        Literal::String(t) => {
            let v = t.value.parse::<String>().unwrap();
            pgrm.instructions.push(Instruction::String(v));
        }
        Literal::Identifier(_) => {
            /*pgrm.instructions
                .push(Instruction::DupPlusFP(locals[&ident.value]));*/
        }
    }
}

fn compile_expression(
    pgrm: &mut Program,
    raw: &[char],
    exp: Expression,
) {
    match exp {
        Expression::BinaryOperation(_) => {
            //compile_binary_operation(pgrm, raw, locals, bop);
        }
        Expression::FunctionCall(fc) => {
            compile_function_call(pgrm, raw, fc);
        }
        Expression::Literal(lit) => {
            compile_literal(pgrm, raw, lit);
        }
    }
}

fn compile_declare(
    pgrm: &mut Program,
    raw: &[char],
    declare: Declare,
) {
    pgrm.instructions.push(Instruction::Variable(declare.name.value));
    compile_expression(pgrm, raw, declare.expression);

    pgrm.instructions.push(Instruction::DeclareEnd);
}

fn compile_function(
    pgrm: &mut Program,
    raw: &[char],
    fd: FunctionDeclaration,
) {
    pgrm.instructions.push(Instruction::Function(fd.name.value.to_owned(), fd.parameters.len()));

    for statement in fd.body {
        compile_statement(pgrm, raw, statement);
    }

    pgrm.instructions.push(Instruction::FunctionEnd);
}

fn compile_statement(
    pgrm: &mut Program,
    raw: &[char],
    stmt: Statement,
) {
    match stmt {
        Statement::FunctionDeclaration(fd) => compile_function(pgrm, raw, fd),
        Statement::Declare(loc) => compile_declare(pgrm, raw, loc),
        Statement::Expression(e) => compile_expression(pgrm, raw, e),
    }
}

pub fn compile(raw: &[char], ast: Ast) -> Program {
    let mut pgrm = Program {
        instructions: Vec::new(),
    };

    for stmt in ast {
        compile_statement(&mut pgrm, raw, stmt);
    }

    pgrm
}