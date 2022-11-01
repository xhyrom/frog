use self::expect::*;
use self::parse::*;

use super::lexer::*;

pub mod expect;
pub mod parse;

#[derive(Debug)]
pub enum Literal {
    Identifier(Token),
    Number(Token),
    String(Token)
}

#[derive(Debug)]
pub struct FunctionCall {
    pub name: Token,
    pub arguments: Vec<Expression>,
}

#[derive(Debug)]
pub struct BinaryOperation {
    pub operator: Token,
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

#[derive(Debug)]
pub enum Expression {
    FunctionCall(FunctionCall),
    BinaryOperation(BinaryOperation),
    Literal(Literal),
}

#[derive(Debug)]
pub struct FunctionDeclaration {
    pub name: Token,
    pub parameters: Vec<Token>,
    pub body: Vec<Statement>,
}

#[derive(Debug)]
pub struct Declare {
    pub name: Token,
    pub expression: Expression,
}

#[derive(Debug)]
pub enum Statement {
    Expression(Expression),
    FunctionDeclaration(FunctionDeclaration),
    Declare(Declare),
}

pub type Ast = Vec<Statement>;

fn parse_statement(raw: &[char], tokens: &[Token], index: usize) -> Option<(Statement, usize)> {
    let parsers = [
        parse_expression_statement,
        parse_task,
        parse_declare,
    ];
    for parser in parsers {
        let res = parser(raw, tokens, index);
        if res.is_some() {
            return res;
        }
    }

    None
}

pub fn parse(raw: &[char], tokens: Vec<Token>) -> Result<Ast, String> {
    let mut ast = vec![];
    let mut index = 0;
    let ntokens = tokens.len();
    while index < ntokens {
        let res = parse_statement(raw, &tokens, index);
        if let Some((stmt, next_index)) = res {
            index = next_index;
            ast.push(stmt);
            continue;
        }

        return Err(tokens[index].loc.debug(raw, "Invalid token while parsing"));
    }

    Ok(ast)
}