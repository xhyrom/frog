use frog_logger::warn;

/**
 * Base is from https://github.com/eatonphil/lust
 * Under Apache 2.0 license
 */

use super::*;

pub fn parse_expression(raw: &[char], tokens: &[Token], index: usize) -> Option<(Expression, usize)> {
    if index >= tokens.len() {
        return None;
    }
    
    let t = tokens[index].clone();
    let left = match t.kind {
        TokenKind::Number => Expression::Literal(Literal::Number(t)),
        TokenKind::String => Expression::Literal(Literal::String(t)),
        TokenKind::Identifier => Expression::Literal(Literal::Identifier(t)),
        _ => {
            return None;
        }
    };
    let mut next_index = index + 1;
    if expect_syntax(tokens, next_index, "(") {
        next_index += 1; // Skip past open paren

        // Function call
        let mut arguments: Vec<Expression> = vec![];
        while !expect_syntax(tokens, next_index, ")") {
            if !arguments.is_empty() {
                if !expect_syntax(tokens, next_index, ",") {
                    warn!(
                        "{}",
                        tokens[next_index]
                            .loc
                            .debug(raw, "Expected comma between function call arguments")
                    );
                    return None;
                }

                next_index += 1; // Skip past comma
            }

            let res = parse_expression(raw, tokens, next_index);
            if let Some((arg, next_next_index)) = res {
                next_index = next_next_index;
                arguments.push(arg);
            } else {
                warn!(
                    "{}",
                    tokens[next_index]
                        .loc
                        .debug(raw, "Expected valid expression in function call arguments")
                );
                return None;
            }
        }

        next_index += 1; // Skip past closing paren

        return Some((
            Expression::FunctionCall(FunctionCall {
                name: tokens[index].clone(),
                arguments,
            }),
            next_index,
        ));
    }

    // Might be a literal expression
    if next_index >= tokens.len() || tokens[next_index].clone().kind != TokenKind::Operator {
        return Some((left, next_index));
    }

    // Otherwise is a binary operation
    let op = tokens[next_index].clone();
    next_index += 1; // Skip past op

    if next_index >= tokens.len() {
        warn!(
            "{}",
            tokens[next_index]
                .loc
                .debug(raw, "Expected valid right hand side binary operand")
        );
        return None;
    }

    let rtoken = tokens[next_index].clone();
    let right = match rtoken.kind {
        TokenKind::Number => Expression::Literal(Literal::Number(rtoken)),
        TokenKind::Identifier => Expression::Literal(Literal::Identifier(rtoken)),
        _ => {
            warn!(
                "{}",
                rtoken
                    .loc
                    .debug(raw, "Expected valid right hand side binary operand")
            );
            return None;
        }
    };
    next_index += 1; // Skip past right hand operand

    Some((
        Expression::BinaryOperation(BinaryOperation {
            left: Box::new(left),
            right: Box::new(right),
            operator: op,
        }),
        next_index,
    ))
}

pub fn parse_expression_statement(
    raw: &[char],
    tokens: &[Token],
    index: usize,
) -> Option<(Statement, usize)> {
    let mut next_index = index;
    let res = parse_expression(raw, tokens, next_index)?;

    let (expr, next_next_index) = res;
    next_index = next_next_index;
    if !expect_syntax(tokens, next_index, ";") {
        warn!(
            "{}",
            tokens[next_index]
                .loc
                .debug(raw, "Expected semicolon after expression")
        );
        return None;
    }

    next_index += 1; // Skip past semicolon

    Some((Statement::Expression(expr), next_index))
}

pub fn parse_task(raw: &[char], tokens: &[Token], index: usize) -> Option<(Statement, usize)> {
    if !expect_keyword(tokens, index, "task") {
        return None;
    }

    let mut next_index = index + 1; // Skip past task keyword
    if !expect_identifier(tokens, next_index) {
        warn!(
            "{}",
            tokens[next_index]
                .loc
                .debug(raw, "Expected valid identifier for task name")
        );
        return None;
    }
    let name = tokens[next_index].clone();

    next_index += 1; // Skip past name
    if !expect_syntax(tokens, next_index, "(") {
        warn!(
            "{}",
            tokens[next_index]
                .loc
                .debug(raw, "Expected open parenthesis in function declaration")
        );
        return None;
    }

    next_index += 1; // Skip past open paren
    let mut parameters: Vec<Token> = vec![];
    while !expect_syntax(tokens, next_index, ")") {
        if !parameters.is_empty() {
            if !expect_syntax(tokens, next_index, ",") {
                warn!("{}", tokens[next_index].loc.debug(raw, "Expected comma or close parenthesis after parameter in function declaration"));
                return None;
            }

            next_index += 1; // Skip past comma
        }

        parameters.push(tokens[next_index].clone());
        next_index += 1; // Skip past param
    }

    next_index += 1; // Skip past close paren

    if !expect_syntax(tokens, next_index, "{") {
        warn!(
            "{}",
            tokens[next_index]
                .loc
                .debug(raw, "Expected open parenthesis in function declaration")
        );
        return None;
    }

    next_index += 1; // Skip past open paren

    let mut statements: Vec<Statement> = vec![];
    while !expect_syntax(tokens, next_index, "}") {
        let res = parse_statement(raw, tokens, next_index);
        if let Some((stmt, next_next_index)) = res {
            next_index = next_next_index;
            statements.push(stmt);
        } else {
            warn!(
                "{}",
                tokens[next_index]
                    .loc
                    .debug(raw, "Expected valid statement in function declaration")
            );
            return None;
        }
    }

    next_index += 1; // Skip past end

    Some((
        Statement::FunctionDeclaration(FunctionDeclaration {
            name,
            parameters,
            body: statements,
        }),
        next_index,
    ))
}

pub fn parse_declare(raw: &[char], tokens: &[Token], index: usize) -> Option<(Statement, usize)> {
    if !expect_keyword(tokens, index, "declare") {
        return None;
    }

    let mut next_index = index + 1; // Skip past declare

    if !expect_identifier(tokens, next_index) {
        warn!(
            "{}",
            tokens[next_index]
                .loc
                .debug(raw, "Expected valid identifier for declare name")
        );
        return None;
    }

    let name = tokens[next_index].clone();
    next_index += 1; // Skip past name

    if !expect_syntax(tokens, next_index, "=") {
        warn!(
            "{}",
            tokens[next_index]
                .loc
                .debug(raw, "Expected = syntax after local name")
        );
        return None;
    }

    next_index += 1; // Skip past =

    let res = parse_expression(raw, tokens, next_index);
    if res.is_none() {
        warn!(
            "{}",
            tokens[next_index]
                .loc
                .debug(raw, "Expected valid expression in local declaration")
        );
        return None;
    }

    let (expr, next_next_index) = res.unwrap();
    next_index = next_next_index;

    if !expect_syntax(tokens, next_index, ";") {
        warn!(
            "{}",
            tokens[next_index]
                .loc
                .debug(raw, "Expected semicolon in return statement")
        );
        return None;
    }

    next_index += 1; // Skip past semicolon

    Some((
        Statement::Declare(Declare {
            name,
            expression: expr,
        }),
        next_index,
    ))
}
