use crate::scanner::TokenType;
use crate::scanner::{self, Token};
use core::panic;
use miette::{Error, LabeledSpan, Severity, miette};
use std::collections::TryReserveError;
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

    pub fn current(&mut self) -> Token<'a> {
        self.stream.get(self.pos - 1 ).expect("Invariant broken: if peek reached None that means that the prev token must have been EOF and was not caught.").clone()
    }
    // behave like peek()
    pub fn peek(&mut self) -> Token<'a> {
        self.stream.get(self.pos).expect("Invariant broken: if peek reached None that means that the prev token must have been EOF and was not caught.").clone()
    }
    fn parse_print(&mut self) -> Result<Tree<'a>, Error> {
        let value = self.peek();
        if value.kind != TokenType::String {
            return Err(miette::miette!(
                help = "This is a syntax error",
                labels = vec![LabeledSpan::at_offset(0, "here")],
                "Unexpected Token"
            ));
        } else if self.expect_semicolon() {
            self.advance();
            return Ok(Tree::Atom(Atom::String(value.lexeme)));
        } else {
            let source = String::from(value.lexeme);
            return Err(miette!(
                severity = Severity::Error,
                help = "This is a syntax error",
                labels = vec![LabeledSpan::at_offset(value.lexeme.len() - 1, "here")],
                "Missing Semicolon ';'"
            )
            .with_source_code(source));
        };
    }

    //TODO: write a expect function
    fn expect(&mut self, input: Token<'a>) -> Result<bool, Error> {
        if self.peek().kind != input.kind {
            Err(miette::miette!(
                help = "This is a syntax error",
                labels = vec![LabeledSpan::at_offset(0, "here")],
                "Unexpected Token"
            ))
        } else {
            Ok(true)
        }
    }

    fn expect_semicolon(&mut self) -> bool {
        if self.peek().kind != TokenType::Semicolon {
            false
        } else {
            true
        }
    }
    pub fn parse_statment(&mut self) -> Result<Tree<'a>, Error> {
        println!("{:?}", self.peek());
        let lhs = match self.advance() {
            n @ Token {
                kind: TokenType::Print,
                ..
            } => self.parse_print()?,
            _ => {
                todo!()
            }
        };
        Ok(lhs)
    }
    // make a lhs and call parse_inner
    pub fn parse_expr(&mut self, min_bp: u8) -> Result<Tree<'a>, Error> {
        let mut lhs = match self.advance() {
            n @ Token {
                kind: TokenType::Plus | TokenType::Minus | TokenType::Bang,
                ..
            } => {
                let ((), bp) = prefix_binding_power(&n.kind);
                let rhs = self.parse_expr(bp)?;
                Tree::NonTerm(n.kind, vec![rhs])
            }
            n @ Token {
                kind: TokenType::LeftParen,
                ..
            } => {
                let lhs = self.parse_expr(0)?;
                lhs
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
                    kind:
                        TokenType::Plus
                        | TokenType::LeftBrace
                        | TokenType::LeftHardBrace
                        | TokenType::Minus
                        | TokenType::Slash
                        | TokenType::Star,
                    ..
                } => n,
                _ => {
                    break;
                }
            };

            //NOTE: I don't belive the standard lox language has postfix but
            //I might add some in the future
            if let Some((l_bp, ())) = postfix_binding_power(&op.kind) {
                if l_bp < min_bp {
                    break;
                }
                self.advance();
                lhs = if op.kind == TokenType::LeftHardBrace {
                    let temp = op.lexeme.to_string();
                    let rhs = self.parse_expr(0)?;
                    if self.peek().kind != TokenType::RightHardBrace {
                        panic!();
                        let source = String::from(format!("{temp} ... EXPECTING CLOSING: ]"));
                        return Err(miette!(
                            severity = Severity::Error,
                            help = "This is a syntax error",
                            labels = vec![LabeledSpan::at_offset(0, "here")],
                            "Missing Closing ]"
                        )
                        .with_source_code(source));
                    }
                    Tree::NonTerm(op.kind, vec![lhs, rhs])
                } else {
                    Tree::NonTerm(op.kind, vec![lhs])
                };
                continue;
            }

            //INFIX
            if let Some((l_bp, r_bp)) = infix_binding_power(&op.kind) {
                println!("l_bp: {:?}", l_bp);
                if l_bp < min_bp {
                    break;
                }
                self.advance();
                let rhs = self.parse_expr(r_bp)?;
                println!("rhs: {:?}", rhs);
                lhs = Tree::NonTerm(op.kind, vec![lhs, rhs]);

                continue;
            }
            //FOR: 2 children, a bracket and a brace

            // if let Some((l_bp, r_bp)) = keywords(&op.kind) {
            //     println!("l_bp: {:?}", l_bp);
            //     if l_bp < min_bp {
            //         break;
            //     }
            //     self.advance();
            //     let rhs = self.parse_expr(r_bp)?;
            //     println!("rhs: {:?}", rhs);
            //     lhs = Tree::NonTerm(op.kind, vec![lhs, rhs]);
            //
            //     continue;
            // }
            break;
        }
        Ok(lhs)
    }
}

// fn logical_operators(op: &scanner::TokenType) -> ((), u8) {}
//
// fn keywords(op: &scanner::TokenType) -> Option<(u8, ())> {
//     match op {
//         TokenType::For | TokenType::Else => Some(((), 2)),
//         TokenType::Class | TokenType::Fun => Some(((), 2)),
//         TokenType::While | TokenType::Var => Some(((), 2)),
//         TokenType::This | TokenType::Print => Some(((), 2)),
//         TokenType::Super | TokenType::Return => Some(((), 2)),
//         TokenType::And | TokenType::If => Some(((), 4)),
//         _ => None,
//     }
// }

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
        TokenType::LeftHardBrace => Some((11, ())),
        _ => None,
    }
}

impl<'a> std::fmt::Display for Tree<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tree::Atom(x) => {
                write!(f, "{:?}", x)
            }
            Tree::NonTerm(parent, children) => {
                //NOTE: remeber the error u had
                //here this was really interesting (stack overflow)
                write!(f, "({}", parent)?;
                for i in children {
                    write!(f, ", {}", i)?
                }
                write!(f, ")")
            }
            _ => {
                unimplemented!()
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
