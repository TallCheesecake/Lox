use crate::scanner::TokenType;
use crate::scanner::{self, Token};
use core::panic;
use miette::{Error, LabeledSpan};

enum Tree<'a> {
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
        TokenType::Plus => (Atom::String(input.lexeme.clone()), true),
        TokenType::Nil => (Atom::String(input.lexeme.clone()), true),
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
    pub fn construct(input: &'a str) -> Parser<'a> {
        Self {
            scanner: scanner::collect(input),
            pos: 0,
        }
    }

    fn advance(&'a mut self) -> Option<&'a mut Result<Token<'a>, Error>> {
        self.pos += 1;
        self.scanner.get_mut(self.pos)
    }

    fn peek_one(&'a self) -> Option<&'a Result<Token<'a>, Error>> {
        self.scanner.get(1)
    }

    fn make(&'a mut self) -> Tree {
        let lhs = match self.advance() {
            Some(Ok(x)) => {
                if let (atom, true) = is_atom(x) {
                    Tree::Atom(atom)
                } else if let (op, true) = is_op(x) {
                    Tree::Op(op)
                } else {
                    panic!("invalid token: {}", x)
                }
            }
            None => todo!(),
            Some(Err(e)) => todo!(),
        };
        lhs
    }

    fn parse_expresion(&'a mut self, min_bp: u8) -> miette::Result<Tree<'a>, Error> {
        let mut lhs = self.make();
        loop {
            let op = self.peek_one();
            //peek and the subsiquint block checks for None and Err
            let op = match op.expect("critical error") {
                Err(_) => unreachable!(),
                Ok(Token {
                    lexeme: op @ ("+" | "/" | "-" | "*"),
                    ..
                }) => {
                    if let Some((l_bp, r_bp)) = postfix_binding_power(op) {
                        if l_bp < min_bp {
                            break;
                        }
                        self.advance();
                        lhs = if op == &"[" {
                            let rhs = match self.parse_expresion(0) {
                                Ok(x) => x,
                                Err(_) => todo!(),
                            };
                            //since postfix_binding returns none for a ] we need to
                            //manually both assert and mutate to move the thing onto the
                            //closing ]
                            //
                            match self.advance() {
                                None => {
                                    todo!()
                                }
                                //this is a lexing error
                                Some(Err(e)) => return Err(e.wrap_err("lexing error")),
                                Some(Ok(x)) => assert_eq!(x.kind, TokenType::RightBrace),
                            };
                            Tree::NonTerm(op, vec![lhs, rhs])
                            //the above will only be the case if there is a m
                        } else {
                            Tree::NonTerm(op, vec![lhs])
                        };
                        continue;
                    }

                    if let Some((l_bp, r_bp)) = infix_binding_power(op) {
                        if l_bp < min_bp {
                            break;
                        }
                        //peek asserts for us
                        self.advance();

                        let rhs = match self.parse_expresion(r_bp) {
                            Ok(x) => x,
                            Err(_) => todo!(),
                        };
                        Tree::NonTerm(op, vec![lhs, rhs])
                    } else {
                        continue;
                    }
                }
                t => {
                    todo!()
                }
                Ok(tok) => {
                    return Err(miette::miette! {
                    labels = vec![LabeledSpan::at_offset(tok.lexeme.len()+ tok.lexeme.len(),"here")],
                    "unexpected: Token, require operator (-,+, /, *)",
                    });
                }
            };
            break;
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
