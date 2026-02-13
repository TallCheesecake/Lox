use crate::scanner::TokenType;
use crate::scanner::{self, Token};
use core::panic;
use miette::{Error, LabeledSpan};
use std::fmt::Display;

#[derive(Debug)]
pub enum Tree<'a> {
    Nil,
    Atom(Atom<'a>),
    NonTerm(&'a str, Vec<Tree<'a>>),
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
    pub scanner: Vec<Result<Token<'a>, Error>>,
    pub pos: usize,
}

fn is_atom<'a>(input: &scanner::Token<'a>) -> (Atom<'a>, bool) {
    match input.kind {
        TokenType::Number(x) => (Atom::Number(x.clone()), true),
        TokenType::True => (Atom::Bool(true), true),
        TokenType::False => (Atom::Bool(false), true),
        TokenType::Super => (Atom::Super, true),
        TokenType::Star => (Atom::This, true),
        TokenType::Slash => (Atom::Nil, true),
        TokenType::Plus => (Atom::String(input.lexeme), true),
        TokenType::Nil => (Atom::String(input.lexeme), true),
        _ => todo!(),
    }
}

fn is_op<'a>(input: &'a scanner::Token<'a>) -> (Op, bool) {
    match input.kind {
        TokenType::Minus => (Op::Minus, true),
        TokenType::Star => (Op::Star, true),
        TokenType::Slash => (Op::Slash, true),
        TokenType::Plus => (Op::Plus, true),
        _ => todo!(),
    }
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Parser<'a> {
        Parser {
            scanner: scanner::collect(input),
            pos: 0,
        }
    }

    fn current(&self) -> Result<&Token<'a>, &Error> {
        self.scanner.get(self.pos).unwrap().as_ref()
    }

    fn peek(&self) -> Option<&Result<Token<'a>, Error>> {
        self.scanner.get(self.pos)
    }

    fn advance(&mut self) -> Result<&Token<'a>, &Error> {
        let current = self.current();
        current
    }

    pub fn parse_expresion(&mut self, min_bp: u8) -> miette::Result<Tree<'a>, Error> {
        let mut lhs = match self.advance() {
            Ok(Token {
                kind: TokenType::Number(e),
                ..
            }) => Tree::Atom(Atom::Number(e.clone())),

            Ok(Token {
                kind: TokenType::Star | TokenType::Plus,
                ..
            }) => Tree::Op(Op::Plus),

            Err(e) => {
                todo!()
            }
            x => {
                println!("x value: {:?}", x);
                panic!()
            }
        };
        // match lhs {
        //     Tree::Atom(_) => self.pos += 1,
        //     _ => {}
        // };
        self.pos += 1;

        loop {
            let op_result = match self.peek() {
                Some(Ok(Token {
                    kind: TokenType::Eof,
                    ..
                })) => {
                    break;
                }
                Some(Ok(tok)) => tok.lexeme,
                Some(Err(_)) => {
                    break;
                }
                None => {
                    println!("none");
                    break;
                }
            };

            match op_result {
                op @ ("+" | "-" | "/" | "*") => {
                    if let Some((l_bp, r_bp)) = infix_binding_power(op) {
                        if l_bp < min_bp {
                            break;
                        }
                        self.advance();
                        self.pos += 1;
                        let rhs = self.parse_expresion(r_bp)?;
                        lhs = Tree::NonTerm(op, vec![lhs, rhs]);
                    };
                    continue;
                }
                "[" => {
                    if let Some((l_bp, _)) = postfix_binding_power("[") {
                        if l_bp < min_bp {
                            break;
                        }
                        self.advance();

                        let rhs = self.parse_expresion(0)?;

                        match self.advance() {
                            Ok(tok) if tok.lexeme == "]" => {
                                lhs = Tree::NonTerm("[", vec![lhs, rhs]);
                            }
                            Ok(tok) => {
                                return Err(miette::miette! {
                                    labels = vec![LabeledSpan::at_offset(tok.lexeme.len(), "here")],
                                    "expected ']', found '{}'",
                                    tok.lexeme
                                });
                            }
                            Err(_) => break, // _ => {
                        }
                    }
                    continue;
                }
                _ => break,
            }
        }
        Ok(lhs)
    }
}

//this is a really good idea from: https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html
fn prefix_binding_power(op: &str) -> Option<((), u8)> {
    assert!(op.len() == 1);
    //The assert garantees
    let op = op.chars().next().expect("critical error");
    match op {
        '+' | '-' => Some(((), 5)),
        _ => None,
    }
}

fn infix_binding_power(op: &str) -> Option<(u8, u8)> {
    assert!(op.len() == 1);
    //The assert garantees
    let op = op.chars().next().expect("critical error");
    match op {
        '+' | '-' => Some((1, 2)),
        '*' | '/' => Some((3, 4)),
        _ => None,
    }
}

fn postfix_binding_power(op: &str) -> Option<(u8, ())> {
    assert!(op.len() == 1);
    //The assert garantees
    let op = op.chars().next().expect("critical error");
    match op {
        '!' => Some((11, ())),
        '[' => Some((11, ())),
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
