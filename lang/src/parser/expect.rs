use super::*;

pub fn expect_keyword(tokens: &[Token], index: usize, value: &str) -> bool {
    if index >= tokens.len() {
        return false;
    }

    let t = tokens[index].clone();
    t.kind == TokenKind::Keyword && t.value == value
}

pub fn expect_syntax(tokens: &[Token], index: usize, value: &str) -> bool {
    if index >= tokens.len() {
        return false;
    }

    let t = tokens[index].clone();
    t.kind == TokenKind::Syntax && t.value == value
}

pub fn expect_identifier(tokens: &[Token], index: usize) -> bool {
    if index >= tokens.len() {
        return false;
    }

    let t = tokens[index].clone();
    t.kind == TokenKind::Identifier
}