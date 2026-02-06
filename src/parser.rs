use crate::scanner;
use crate::scanner::MyBad;
use crate::scanner::Token;
use crate::scanner::TokenType;
use miette::{Result, WrapErr};
enum Ops {
    Add,
    Bang,
    Div,
    Mul,
    Sub,
}

enum Exp<'a> {
    Term(Token<'a>),
    NonTerm(Token<'a>, Vec<Exp<'a>>),
}
enum Tree<'a> {
    Nil,
    Atom(Atom<'a>),
    NonTerm(&'a str, Vec<Tree<'a>>),
}
#[derive(Debug, Clone, PartialEq)]
//NOTE:these are things that canot derive, they are non terminal
//String canot derive anything, unlike for example func which can derive into the
//remaining function
pub enum Atom<'a> {
    String(&'a str),
    Number(f64),
    Nil,
    Bool(bool),
    Ident(&'a str),
    Super,
    This,
}
pub struct Parser<'a> {
    pub scanner: scanner::Scanner<'a>,
}

impl<'a> Parser<'a> {
    pub fn construct(input: &'a str) -> Parser<'a> {
        Self {
            scanner: scanner::Scanner::new(input),
        }
    }
    //This kinda stucture was inspired by: https://github.com/jonhoo/lox/blob/master/src/parse.rs#L426
    pub fn parse_expresion(&mut self, min_bp: u8) -> miette::Result<Tree> {
        let mut lhs = match self.scanner.next() {
            //I need a helper that will figure out what this token is
            //with respect to the tree
            Some(Ok(x)) => Tree::Nil,
            None => return Ok(Tree::Nil),
            Some(Err(e)) => return Err(e).wrap_err("error in token fields"),
        };

        loop {
            let op = match self.scanner.next() {
                Some(Ok(x)) => {
                    if matches!(x.kind, TokenType::Eof) {
                        break;
                    } else if matches!(
                        x.kind,
                        TokenType::Minus | TokenType::Slash | TokenType::Star | TokenType::Plus
                    ) {
                        x.lexeme
                    } else {
                        panic!()
                    }
                }
                None => {
                    todo!();
                    // Ok(Tree::Nil);
                }
                Some(Err(e)) => return Err(e).wrap_err("error in token fields"),
            };
            let (left_bp, right_bp) = infix_binding_power(op);
            if left_bp < min_bp {
                break;
            }
            self.scanner.next();
            let rhs = self.parse_expresion(right_bp);

            let rhs = match self.parse_expresion(right_bp) {
                (Ok(x)) => Tree::Nil,
                Err(e) => return Err(e).wrap_err("error in token fields"),
            };
            lhs = Tree::NonTerm(op, vec![lhs, rhs]);
        }
        Ok(lhs)
    }
}

//this is a really good idea from: https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html
//it is effectivly manually weighting the binding power of
//any operator which is very convinient
fn infix_binding_power(op: &str) -> (u8, u8) {
    //This is really hacky but I dont care, if we have gotten that
    //far we say this is a invariant
    assert!(op.len() == 1);
    let op = op.chars().next().unwrap_or_else(|| '+');
    match op {
        '+' | '-' => (1, 2),
        '*' | '/' => (3, 4),
        _ => panic!("bad op: {:?}", op),
    }
}

// fn infix_bp(token: scanner::Token) -> u8 {}
