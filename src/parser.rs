use crate::scanner::TokenType;
use crate::scanner::{self, Token};
use core::panic;
use miette::{Error, LabeledSpan, Severity, miette};
use std::fmt::Display;

#[derive(Debug)]
pub enum Tree<'a> {
    Nil,
    Atom(Atom<'a>),
    NonTerm(scanner::TokenType, Vec<Tree<'a>>),
    Op(Op),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Atom<'a> {
    String(&'a str),
    Number(f64),
    Nil,
    Bool(bool),
    Ident(&'a str),
    Super,
    This,
    Error,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Op {
    Minus,
    Plus,
    Star,
    BangEqual,
    EqualEqual,
    LessEqual,
    GreaterEqual,
    Less,
    Greater,
    Slash,
    Bang,
    And,
    Or,
    Call,
    For,
    Class,
    Print,
    Return,
    Field,
    Var,
    While,
    Group,
}

#[derive(Debug)]
pub struct Parser<'a> {
    pub stream: Vec<Token<'a>>,
    pub pos: usize,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Result<Parser<'a>, Error> {
        let scanner = scanner::collect(input)?;
        Ok(Parser {
            stream: scanner,
            pos: 0,
        })
    }
    pub fn advance(&mut self) -> Token<'a> {
        let output = self.stream.get(self.pos).expect("Invariant broken: should not be possible for advance to return none since the prev match should break out on EOF.").clone();
        self.pos += 1;
        output
    }

    // behave like peek()
    pub fn peek(&mut self) -> Token<'a> {
        self.stream.get(self.pos).expect("Invariant broken: if peek reached None that means that the prev token must have been EOF and was not caught.").clone()
    }

    // make a lhs and call parse_inner
    pub fn parse_expr(&mut self, min_bp: u8) -> Result<Tree<'a>, Error> {
        let mut lhs = match self.advance() {
            //TODO: Learn more about this syntax
            n @ Token {
                kind: TokenType::Plus | TokenType::Minus,
                ..
            } => {
                let ((), bp) = prefix_binding_power(&n.kind);
                let rhs = self.parse_expr(bp)?;
                Tree::NonTerm(n.kind, vec![rhs])
            }
            n @ Token {
                kind: TokenType::Identifier,
                ..
            } => Tree::Atom(Atom::Ident(n.lexeme)),
            Token {
                kind: TokenType::Number(a),
                ..
            } => Tree::Atom(Atom::Number(a as f64)),
            e => {
                let source = String::from(e.lexeme);
                return Err(miette!(
                    severity = Severity::Error,
                    help = "This is a syntax error",
                    labels = vec![LabeledSpan::at_offset(0, "here")],
                    "Unexpected Token"
                )
                .with_source_code(source));
            }
        };
        loop {
            println!("peek: {}", self.peek());

            let op = match self.peek() {
                Token {
                    kind: TokenType::Eof,
                    ..
                } => {
                    break;
                }
                n @ Token {
                    kind: TokenType::Plus | TokenType::Minus | TokenType::Slash | TokenType::Star,
                    ..
                } => n,
                _ => {
                    break;
                }
            };

            if let Some((l_bp, r_bp)) = infix_binding_power(&op.kind) {
                println!("l_bp: {:?}", l_bp);
                if l_bp < min_bp {
                    println!("broke in compar");
                    break;
                }
                self.advance();
                let rhs = self.parse_expr(r_bp)?;
                println!("rhs: {:?}", rhs);
                lhs = Tree::NonTerm(op.kind, vec![lhs, rhs]);

                continue;
            }
            break;
        }
        Ok(lhs)
    }
}

fn prefix_binding_power(op: &scanner::TokenType) -> ((), u8) {
    match op {
        TokenType::Plus | TokenType::Minus => ((), 5),
        _ => {
            panic!("woops bad token this should be a error")
        }
    }
}

fn infix_binding_power(op: &scanner::TokenType) -> Option<(u8, u8)> {
    //The assert garantees
    match op {
        TokenType::Plus | TokenType::Minus => Some((1, 2)),
        TokenType::Star | TokenType::Slash => Some((3, 4)),
        _ => None,
    }
}

fn postfix_binding_power(op: &scanner::TokenType) -> Option<(u8, ())> {
    //The assert garantees
    match op {
        TokenType::Bang => Some((11, ())),
        TokenType::LeftBrace => Some((11, ())),
        _ => None,
    }
}

impl<'a> std::fmt::Display for Tree<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            r => {
                write!(f, "{:?}", r)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        unreachable!()
    }
}
