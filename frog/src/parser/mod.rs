use std::fmt;

use crate::{ast::*, lexer::Lexer, token::Token};

#[derive(Debug, Clone)]
pub enum ParseErrorKind {
    UnexpectedToken,
}

impl fmt::Display for ParseErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParseErrorKind::UnexpectedToken => write!(f, "Unexpected Token"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ParseError {
    kind: ParseErrorKind,
    msg: String,
}

impl ParseError {
    fn new(kind: ParseErrorKind, msg: String) -> Self {
        ParseError { kind, msg }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.kind, self.msg)
    }
}

pub type ParseErrors = Vec<ParseError>;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    previous_token: Token,
    current_token: Token,
    next_token: Token,
    errors: ParseErrors,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        let mut parser = Parser {
            lexer,
            previous_token: Token::Eof,
            current_token: Token::Eof,
            next_token: Token::Eof,
            errors: vec![],
        };

        parser.bump();
        parser.bump();

        parser
    }

    fn token_to_precedence(tok: &Token) -> Precedence {
        match tok {
            Token::Equal | Token::NotEqual => Precedence::Equals,
            Token::LessThan | Token::LessThanEqual => Precedence::LessGreater,
            Token::GreaterThan | Token::GreaterThanEqual => Precedence::LessGreater,
            Token::Plus | Token::Minus => Precedence::Sum,
            Token::Slash | Token::Asterisk => Precedence::Product,
            Token::Dot | Token::Lbracket => Precedence::Index,
            Token::Lparen => Precedence::Call,
            _ => Precedence::Lowest,
        }
    }

    pub fn get_errors(&mut self) -> ParseErrors {
        self.errors.clone()
    }

    fn bump(&mut self) {
        self.previous_token = self.current_token.clone();
        self.current_token = self.next_token.clone();
        self.next_token = self.lexer.next_token();
    }

    fn previous_token_is(&mut self, tok: &Token) -> bool {
        self.previous_token == *tok
    }

    fn current_token_is(&mut self, tok: Token) -> bool {
        self.current_token == tok
    }

    fn next_token_is(&mut self, tok: &Token) -> bool {
        self.next_token == *tok
    }

    fn expect_next_token(&mut self, tok: Token) -> bool {
        if self.next_token_is(&tok) {
            self.bump();
            return true;
        } else {
            self.error_next_token(tok);
            return false;
        }
    }

    fn current_token_precedence(&mut self) -> Precedence {
        Self::token_to_precedence(&self.current_token)
    }

    fn next_token_precedence(&mut self) -> Precedence {
        Self::token_to_precedence(&self.next_token)
    }

    fn error_next_token(&mut self, tok: Token) {
        self.errors.push(ParseError::new(
            ParseErrorKind::UnexpectedToken,
            format!(
                "expected next token to be {:?}, got {:?} instead",
                tok, self.next_token
            ),
        ));
    }

    fn error_no_prefix_parser(&mut self) {
        self.errors.push(ParseError::new(
            ParseErrorKind::UnexpectedToken,
            format!(
                "no prefix parse function for  \"{:?}\" found",
                self.current_token,
            ),
        ));
    }

    pub fn parse(&mut self) -> Program {
        let mut program: Program = vec![];

        while !self.current_token_is(Token::Eof) {
            match self.parse_stmt() {
                Some(stmt) => program.push(stmt),
                None => {}
            }
            self.bump();
        }

        program
    }

    fn parse_block_stmt(&mut self) -> BlockStmt {
        self.bump();

        let mut block = vec![];

        while !self.current_token_is(Token::Rbrace) && !self.current_token_is(Token::Eof) {
            match self.parse_stmt() {
                Some(stmt) => block.push(stmt),
                None => {}
            }
            self.bump();
        }

        block
    }

    fn parse_stmt(&mut self) -> Option<Stmt> {
        match self.current_token {
            Token::Pub => self.parse_pub_stmt(),
            Token::Let => self.parse_let_stmt(),
            Token::Import => self.parse_import_stmt(),
            Token::Return => self.parse_return_stmt(),
            Token::Blank => Some(Stmt::Blank),
            _ => self.parse_expr_stmt(),
        }
    }

    fn parse_pub_stmt(&mut self) -> Option<Stmt> {
        self.bump();

        match self.current_token {
            Token::Let => self.parse_let_stmt(),
            Token::Func => self.parse_expr_stmt(),
            _ => None,
        }
    }

    fn parse_let_stmt(&mut self) -> Option<Stmt> {
        let public = self.previous_token_is(&Token::Pub);

        match &self.next_token {
            Token::Ident(_) => self.bump(),
            _ => return None,
        };

        let name = match self.parse_ident() {
            Some(name) => name,
            None => return None,
        };

        if !self.expect_next_token(Token::Assign) {
            return None;
        }

        self.bump();

        let expr = match self.parse_expr(Precedence::Lowest) {
            Some(expr) => expr,
            None => return None,
        };

        if self.next_token_is(&Token::Semicolon) {
            self.bump();
        }

        Some(Stmt::Let(name, expr, public))
    }

    fn parse_import_stmt(&mut self) -> Option<Stmt> {
        match &self.next_token {
            Token::String(_) => self.bump(),
            _ => return None,
        };

        let expr = match self.parse_string_expr() {
            Some(expr) => expr,
            None => return None,
        };

        if self.next_token_is(&Token::Semicolon) {
            self.bump();
        }

        Some(Stmt::Import(expr))
    }

    fn parse_return_stmt(&mut self) -> Option<Stmt> {
        self.bump();

        let expr = match self.parse_expr(Precedence::Lowest) {
            Some(expr) => expr,
            None => return None,
        };

        if self.next_token_is(&Token::Semicolon) {
            self.bump();
        }

        Some(Stmt::Return(expr))
    }

    fn parse_expr_stmt(&mut self) -> Option<Stmt> {
        match self.parse_expr(Precedence::Lowest) {
            Some(expr) => {
                if self.next_token_is(&Token::Semicolon) {
                    self.bump();
                }
                Some(Stmt::Expr(expr))
            }
            None => None,
        }
    }

    fn parse_expr(&mut self, precedence: Precedence) -> Option<Expr> {
        // prefix
        let mut left = match self.current_token {
            Token::Ident(_) => self.parse_ident_expr(),
            Token::Int(_) => self.parse_int_expr(),
            Token::Float(_) => self.parse_float_expr(),
            Token::String(_) => self.parse_string_expr(),
            Token::Char(_) => self.parse_char_expr(),
            Token::Bool(_) => self.parse_bool_expr(),
            Token::Lbracket => self.parse_array_expr(),
            Token::Lbrace => self.parse_hash_expr(),
            Token::Bang | Token::Minus | Token::Plus => self.parse_prefix_expr(),
            Token::Lparen => self.parse_grouped_expr(),
            Token::Dot => None,
            Token::If => self.parse_if_expr(),
            Token::Func => self.parse_func_expr(),
            _ => {
                self.error_no_prefix_parser();
                return None;
            }
        };

        // infix
        while !self.next_token_is(&Token::Semicolon) && precedence < self.next_token_precedence() {
            match self.next_token {
                Token::Plus
                | Token::Minus
                | Token::Slash
                | Token::Asterisk
                | Token::Equal
                | Token::NotEqual
                | Token::LessThan
                | Token::LessThanEqual
                | Token::GreaterThan
                | Token::GreaterThanEqual => {
                    self.bump();
                    left = self.parse_infix_expr(left.unwrap());
                }
                Token::Dot => {
                    self.bump();
                    left = self.parse_dot_expr(left.unwrap());
                }
                Token::Lbracket => {
                    self.bump();
                    left = self.parse_index_expr(left.unwrap());
                }
                Token::Lparen => {
                    self.bump();
                    left = self.parse_call_expr(left.unwrap());
                }
                _ => return left,
            }
        }

        left
    }

    fn parse_ident(&mut self) -> Option<Ident> {
        match self.current_token {
            Token::Ident(ref mut ident) => Some(Ident(ident.clone())),
            _ => None,
        }
    }

    fn parse_ident_expr(&mut self) -> Option<Expr> {
        match self.parse_ident() {
            Some(ident) => Some(Expr::Ident(ident)),
            None => None,
        }
    }

    fn parse_int_expr(&mut self) -> Option<Expr> {
        match self.current_token {
            Token::Int(ref mut int) => Some(Expr::Literal(Literal::Int(int.clone()))),
            _ => None,
        }
    }

    fn parse_float_expr(&mut self) -> Option<Expr> {
        match self.current_token {
            Token::Float(ref mut float) => Some(Expr::Literal(Literal::Float(float.clone()))),
            _ => None,
        }
    }

    fn parse_string_expr(&mut self) -> Option<Expr> {
        match self.current_token {
            Token::String(ref mut s) => Some(Expr::Literal(Literal::String(s.clone()))),
            _ => None,
        }
    }

    fn parse_char_expr(&mut self) -> Option<Expr> {
        match self.current_token {
            Token::Char(ref mut s) => Some(Expr::Literal(Literal::Char(s.clone()))),
            _ => None,
        }
    }

    fn parse_bool_expr(&mut self) -> Option<Expr> {
        match self.current_token {
            Token::Bool(value) => Some(Expr::Literal(Literal::Bool(value == true))),
            _ => None,
        }
    }

    fn parse_array_expr(&mut self) -> Option<Expr> {
        match self.parse_expr_list(Token::Rbracket) {
            Some(list) => Some(Expr::Literal(Literal::Array(list))),
            None => None,
        }
    }

    fn parse_hash_expr(&mut self) -> Option<Expr> {
        let mut pairs = Vec::new();

        while !self.next_token_is(&Token::Rbrace) {
            self.bump();

            let key = match self.parse_expr(Precedence::Lowest) {
                Some(expr) => expr,
                None => return None,
            };

            if !self.expect_next_token(Token::Colon) {
                return None;
            }

            self.bump();

            let value = match self.parse_expr(Precedence::Lowest) {
                Some(expr) => expr,
                None => return None,
            };

            pairs.push((key, value));

            if !self.next_token_is(&Token::Rbrace) && !self.expect_next_token(Token::Comma) {
                return None;
            }
        }

        if !self.expect_next_token(Token::Rbrace) {
            return None;
        }

        Some(Expr::Literal(Literal::Hash(pairs)))
    }

    fn parse_expr_list(&mut self, end: Token) -> Option<Vec<Expr>> {
        let mut list = vec![];

        if self.next_token_is(&end) {
            self.bump();
            return Some(list);
        }

        self.bump();

        match self.parse_expr(Precedence::Lowest) {
            Some(expr) => list.push(expr),
            None => return None,
        }

        while self.next_token_is(&Token::Comma) {
            self.bump();
            self.bump();

            match self.parse_expr(Precedence::Lowest) {
                Some(expr) => list.push(expr),
                None => return None,
            }
        }

        if !self.expect_next_token(end) {
            return None;
        }

        Some(list)
    }

    fn parse_prefix_expr(&mut self) -> Option<Expr> {
        let prefix = match self.current_token {
            Token::Bang => Prefix::Not,
            Token::Minus => Prefix::Minus,
            Token::Plus => Prefix::Plus,
            _ => return None,
        };

        self.bump();

        match self.parse_expr(Precedence::Prefix) {
            Some(expr) => Some(Expr::Prefix(prefix, Box::new(expr))),
            None => None,
        }
    }

    fn parse_infix_expr(&mut self, left: Expr) -> Option<Expr> {
        let infix = match self.current_token {
            Token::Plus => Infix::Plus,
            Token::Minus => Infix::Minus,
            Token::Slash => Infix::Divide,
            Token::Asterisk => Infix::Multiply,
            Token::Equal => Infix::Equal,
            Token::NotEqual => Infix::NotEqual,
            Token::LessThan => Infix::LessThan,
            Token::LessThanEqual => Infix::LessThanEqual,
            Token::GreaterThan => Infix::GreaterThan,
            Token::GreaterThanEqual => Infix::GreaterThanEqual,
            _ => return None,
        };

        let precedence = self.current_token_precedence();

        self.bump();

        match self.parse_expr(precedence) {
            Some(expr) => Some(Expr::Infix(infix, Box::new(left), Box::new(expr))),
            None => None,
        }
    }

    fn parse_dot_expr(&mut self, left: Expr) -> Option<Expr> {
        self.bump();

        let index = match self.current_token {
            Token::Ident(ref mut s) => Some(Expr::Literal(Literal::String(s.clone()))),
            _ => return None,
        };

        if index.is_none() {
            return None;
        }

        let index = index.unwrap();

        Some(Expr::Index(Box::new(left), Box::new(index)))
    }

    fn parse_index_expr(&mut self, left: Expr) -> Option<Expr> {
        self.bump();

        let index = match self.parse_expr(Precedence::Lowest) {
            Some(expr) => expr,
            None => return None,
        };

        if !self.expect_next_token(Token::Rbracket) {
            return None;
        }

        Some(Expr::Index(Box::new(left), Box::new(index)))
    }

    fn parse_grouped_expr(&mut self) -> Option<Expr> {
        self.bump();

        let expr = self.parse_expr(Precedence::Lowest);

        if !self.expect_next_token(Token::Rparen) {
            None
        } else {
            expr
        }
    }

    fn parse_if_expr(&mut self) -> Option<Expr> {
        if !self.expect_next_token(Token::Lparen) {
            return None;
        }

        self.bump();

        let cond = match self.parse_expr(Precedence::Lowest) {
            Some(expr) => expr,
            None => return None,
        };

        if !self.expect_next_token(Token::Rparen) || !self.expect_next_token(Token::Lbrace) {
            return None;
        }

        let consequence = self.parse_block_stmt();
        let mut alternative = None;

        if self.next_token_is(&Token::Else) {
            self.bump();

            if !self.expect_next_token(Token::Lbrace) {
                return None;
            }

            alternative = Some(self.parse_block_stmt());
        }

        Some(Expr::If {
            cond: Box::new(cond),
            consequence,
            alternative,
        })
    }

    fn parse_func_expr(&mut self) -> Option<Expr> {
        let public = self.previous_token_is(&Token::Pub);

        match &self.next_token {
            Token::Ident(_) => self.bump(),
            _ => return None,
        };

        let name = match self.parse_ident() {
            Some(name) => name,
            None => return None,
        };

        self.bump();

        let params = match self.parse_func_params() {
            Some(params) => params,
            None => return None,
        };

        if !self.expect_next_token(Token::Lbrace) {
            return None;
        }

        Some(Expr::Func {
            name,
            params,
            body: self.parse_block_stmt(),
            public,
        })
    }

    fn parse_func_params(&mut self) -> Option<Vec<Ident>> {
        let mut params = vec![];

        if self.next_token_is(&Token::Rparen) {
            self.bump();
            return Some(params);
        }

        self.bump();

        match self.parse_ident() {
            Some(ident) => params.push(ident),
            None => return None,
        };

        while self.next_token_is(&Token::Comma) {
            self.bump();
            self.bump();

            match self.parse_ident() {
                Some(ident) => params.push(ident),
                None => return None,
            };
        }

        if !self.expect_next_token(Token::Rparen) {
            return None;
        }

        Some(params)
    }

    fn parse_call_expr(&mut self, func: Expr) -> Option<Expr> {
        let args = match self.parse_expr_list(Token::Rparen) {
            Some(args) => args,
            None => return None,
        };

        Some(Expr::Call {
            func: Box::new(func),
            args,
        })
    }
}
