use crate::scanner::TokenType;
use crate::scanner::{self, Token};
use core::panic;
use miette::{Error, LabeledSpan, Severity, miette};
use std::collections::{HashMap, TryReserveError};

#[derive(Debug)]
pub enum Tree<'a> {
    Nil,
    Call {
        callee: Box<Tree<'a>>,
        arguments: Vec<Tree<'a>>,
    },
    Atom(Atom<'a>),
    NonTerm(Op, Vec<Tree<'a>>),
    Op(Op),
    Fun {
        name: Box<Tree<'a>>,
        parameters: Vec<Tree<'a>>,
        body: Box<Tree<'a>>,
    },
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
impl From<Op> for TokenType {
    fn from(value: Op) -> Self {
        match value {
            Op::Minus => TokenType::Minus,
            Op::Plus => TokenType::Plus,
            Op::Star => TokenType::Star,
            Op::BangEqual => TokenType::BangEqual,
            Op::EqualEqual => TokenType::EqualEqual,
            Op::LessEqual => TokenType::LessEqual,
            Op::GreaterEqual => TokenType::GreaterEqual,
            Op::Less => TokenType::Less,
            Op::Greater => TokenType::Greater,
            Op::Slash => TokenType::Slash,
            Op::Bang => TokenType::Bang,
            Op::And => TokenType::And,
            Op::Or => TokenType::Or,
            Op::For => TokenType::For,
            Op::Class => TokenType::Class,
            Op::Print => TokenType::Print,
            Op::Return => TokenType::Return,
            Op::Var => TokenType::Var,
            Op::While => TokenType::While,
            _ => unreachable!(),
        }
    }
}
impl Into<Op> for TokenType {
    fn into(self) -> Op {
        match self {
            TokenType::LeftParen => Op::Group,
            TokenType::Minus => Op::Minus,
            TokenType::Plus => Op::Plus,
            TokenType::Slash => Op::Slash,
            TokenType::Star => Op::Star,
            TokenType::Bang => Op::Bang,
            TokenType::BangEqual => Op::BangEqual,
            TokenType::EqualEqual => Op::EqualEqual,
            TokenType::Greater => Op::Greater,
            TokenType::GreaterEqual => Op::GreaterEqual,
            TokenType::Less => Op::Less,
            TokenType::LessEqual => Op::LessEqual,
            TokenType::And => Op::And,
            TokenType::Class => Op::Class,
            TokenType::Or => Op::Or,
            TokenType::Print => Op::Print,
            TokenType::Return => Op::Return,
            TokenType::Var => Op::Var,
            TokenType::While => Op::While,
            _ => unreachable!(),
        }
    }
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

// #[derive(Eq, PartialEq, Hash)]
// struct FuncData {
//     name: String,
//     num_args: u8,
// }

// pub struct Functions {
//     pairs: HashMap<FuncIden, >,
// }
//
impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Result<Parser<'a>, Error> {
        let scanner = scanner::collect(input)?;
        Ok(Parser {
            stream: scanner,
            pos: 0,
        })
    }
    // pub fn parse_program(&mut self) -> Result<Tree<'a>, Error> {
    //     let val = self.parse_decl()?;
    //     if !self.expect(TokenType::Eof)? {
    //         todo!()
    //     } else {
    //         Ok(val)
    //     }
    // }
    // pub fn parse_decl(&mut self) -> Result<Tree<'a>, Error> {
    //     match self.advance() {
    //         Token {
    //             kind: TokenType::Fun,
    //             ..
    //         } => {
    //             let val = self.parse_func_decl()?;
    //
    //             if !self.expect(TokenType::Eof)? {
    //                 todo!()
    //             } else {
    //                 Ok(val)
    //             }
    //         }
    //         Token {
    //             kind: TokenType::Var,
    //             ..
    //         } => {
    //             let val = self.parse_func_decl()?;
    //
    //             if !self.expect(TokenType::Eof)? {
    //                 todo!()
    //             } else {
    //                 Ok(val)
    //             }
    //         }
    //         _ => todo!(),
    //     }
    // }
    // // pub fn parse_var_decl(&mut self) -> Result<Tree<'a>, Error> {}
    // // pub fn parse_func_decl(&mut self) -> Result<Tree<'a>, Error> {}
    // // pub fn parse_expr_stmt(&mut self) -> Result<Tree<'a>, Error> {}
    // pub fn parse_call(&mut self) -> Result<Tree<'a>, Error> {
    //     let value = self.peek();
    //     if self.expect(TokenType::Identifier) {
    //         return Ok(Tree::Call {
    //             callee: Box::new(self.parse_expr(0)?),
    //             arguments: vec![self.parse_expr(0)?],
    //         });
    //     };
    //
    //     if value.kind != TokenType::Identifier {
    //         return Err(miette::miette!(
    //             help = "This is a syntax error",
    //             labels = vec![LabeledSpan::at_offset(0, "here")],
    //             "Unexpected Token"
    //         ));
    //     } else if self.expect(TokenType::LeftParen)? {
    //         self.advance();
    //         let lhs = self.parse_expr(0)?;
    //         return Ok(Tree::NonTerm(Op::Call));
    //     } else {
    //         let source = String::from(value.lexeme);
    //         return Err(miette!(
    //             severity = Severity::Error,
    //             help = "This is a syntax error",
    //             labels = vec![LabeledSpan::at_offset(value.lexeme.len() - 1, "here")],
    //             "Missing Semicolon ';'"
    //         )
    //         .with_source_code(source));
    //     };
    // }
    // // pub fn parse_prim(&mut self) -> Result<Tree<'a>, Error> {}
    //
    pub fn parse_expr(&mut self, min_bp: u8) -> Result<Tree<'a>, Error> {
        let mut lhs = match self.advance() {
            Token {
                kind: TokenType::String,
                lexeme,
                ..
            } => Tree::Atom(Atom::String(lexeme)),
            Token {
                kind: TokenType::Number(n),
                ..
            } => Tree::Atom(Atom::Number(n)),
            Token {
                kind: TokenType::True,
                ..
            } => Tree::Atom(Atom::Bool(true)),
            Token {
                kind: TokenType::False,
                ..
            } => Tree::Atom(Atom::Bool(false)),
            Token {
                kind: TokenType::Nil,
                ..
            } => Tree::Atom(Atom::Nil),
            Token {
                kind: TokenType::Identifier,
                lexeme,
                ..
            } => Tree::Atom(Atom::Ident(lexeme)),
            Token {
                kind: TokenType::Super,
                ..
            } => Tree::Atom(Atom::Super),

            Token {
                kind: TokenType::This,
                ..
            } => Tree::Atom(Atom::This),

            n @ Token {
                kind: TokenType::Minus | TokenType::Bang,
                ..
            } => {
                let ((), bp) = prefix_binding_power(&n.kind);
                let rhs = self.parse_expr(bp)?;
                Tree::NonTerm(n.kind.into(), vec![rhs])
            }
            Token {
                kind: TokenType::LeftParen,
                ..
            } => {
                let lhs = self.parse_expr(0)?;
                let source = format!("{} {}", self.prev().lexeme, self.peek().lexeme);
                if !self.expect(TokenType::RightParen) {
                    return Err(miette!(
                        severity = Severity::Error,
                        help = "Always close your parenthesis",
                        labels = vec![LabeledSpan::at_offset(1, "here")],
                        "Expected Right Paren"
                    )
                    .with_source_code(source));
                }
                Tree::NonTerm(Op::Group, vec![lhs])
            }
            e => {
                let source = String::from(e.lexeme);
                return Err(miette!(
                    severity = Severity::Error,
                    help = "hi is a syntax error",
                    labels = vec![LabeledSpan::at_offset(0, "here")],
                    "Unexpected Token"
                )
                .with_source_code(source));
            }
        };
        loop {
            let op = match self.peek() {
                Token {
                    kind:
                        TokenType::Eof
                        | TokenType::Equal
                        | TokenType::RightParen
                        | TokenType::Comma
                        | TokenType::RightBrace
                        | TokenType::Semicolon,
                    ..
                } => {
                    break;
                }

                n @ Token {
                    kind: TokenType::LeftParen,
                    ..
                } => n,
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
                            help = "hello is a syntax error",
                            labels = vec![LabeledSpan::at_offset(0, "here")],
                            "Missing Closing ]"
                        )
                        .with_source_code(source));
                    }

                    Tree::NonTerm(op.kind.into(), vec![lhs, rhs])
                } else {
                    Tree::NonTerm(op.kind.into(), vec![lhs])
                };
                continue;
            }

            //INFIX
            if let Some((l_bp, r_bp)) = infix_binding_power(&op.kind) {
                if l_bp < min_bp {
                    break;
                }
                self.advance();
                let rhs = self.parse_expr(r_bp)?;
                lhs = Tree::NonTerm(op.kind.into(), vec![lhs, rhs]);
                continue;
            }

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

    pub fn prev(&mut self) -> Token<'a> {
        let output = self.stream.get(self.pos - 1).expect("Invariant broken: should not be possible for advance to return none since the prev match should break out on EOF.").clone();
        output
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

    // fn parse_funciton_decl(&mut self, func: &mut Functions) -> Result<Tree<'a>, Error> {
    //     let value = self.peek();
    //     //NOTE: I think that this makes it so that you mush declare before use
    //     if self.expect(TokenType::LeftParen) {
    //         Tree::Atom(Atom::Ident(value.lexeme))
    //     }
    //     if value.kind != TokenType::Identifier {
    //         return Err(miette::miette!(
    //             help = "This is a syntax error",
    //             labels = vec![LabeledSpan::at_offset(0, "here")],
    //             "Unexpected Token"
    //         ));
    //     } else if self.expect(TokenType::LeftParen)? {
    //         self.advance();
    //         let lhs = self.parse_expr(0)?;
    //         return Ok(Tree::NonTerm(Op::Call));
    //     } else {
    //         let source = String::from(value.lexeme);
    //         return Err(miette!(
    //             severity = Severity::Error,
    //             help = "This is a syntax error",
    //             labels = vec![LabeledSpan::at_offset(value.lexeme.len() - 1, "here")],
    //             "Missing Semicolon ';'"
    //         )
    //         .with_source_code(source));
    //     };
    // }
    //
    //TODO: Make print and retrun part of the pratt system as unary prefixes
    pub fn parse_print(&mut self) -> Result<Tree<'a>, Error> {
        let value = self.peek();
        // println!("{:?}", value);
        if value.kind != TokenType::String {
            return Err(miette::miette!(
                help = "This is a syntax error",
                labels = vec![LabeledSpan::at_offset(0, "here")],
                "Unexpected Token"
            ));
        } else if !self.expect_semicolon() {
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

    fn expect(&mut self, input: TokenType) -> bool {
        if self.peek().kind != input {
            false
        } else {
            true
        }
    }

    fn expect_semicolon(&mut self) -> bool {
        if self.peek().kind != TokenType::Semicolon {
            false
        } else {
            true
        }
    }

    pub fn parse_block(&mut self) -> Result<Tree<'a>, Error> {
        let value = self.peek();
        if self.expect(TokenType::LeftBrace) {
            return Err(miette::miette!("expected left paren for block"));
        } else {
            if self.expect(TokenType::RightParen) {
                return Ok(Tree::NonTerm(Op::Group, vec![]));
            }
            return Ok(self.parse_expr(0)?);
        };
    }
    pub fn parse_statment(&mut self) -> Result<Tree<'a>, Error> {
        let lhs = match self.advance() {
            Token {
                kind: TokenType::Print,
                ..
            } => self.parse_print()?,
            Token {
                kind: TokenType::Fun,
                ..
            } => {
                if !self.expect(TokenType::Identifier) {
                    return Err(miette!("expected funciton name"));
                } else {
                    let next = self.peek();
                    self.advance();
                    let func_name = next.lexeme;
                    let func_iden = Tree::Atom(Atom::Ident(func_name));
                    let mut temp = Vec::new();
                    if !self.expect(TokenType::LeftParen) {
                        return Err(miette!("expected paren name"));
                    } else {
                        self.advance();
                        println!("seeeing ( {}", self.current());
                        if self.expect(TokenType::RightParen) {
                            self.advance();
                            {}
                        } else {
                            loop {
                                println!("hello");
                                if self.expect(TokenType::Comma) {
                                    println!("comma");
                                    self.advance();
                                } else if !self.expect(TokenType::Comma)
                                    && self.current().kind != TokenType::RightParen
                                {
                                    temp.push(self.parse_expr(0)?);
                                } else {
                                    break;
                                };
                                self.advance();
                            }
                        };
                    };
                    self.advance();
                    let block = self.parse_block()?;
                    return Ok(Tree::Fun {
                        name: Box::new(func_iden),
                        parameters: temp,
                        body: Box::new(block),
                    });
                };
            }
            Token {
                kind: TokenType::Super,
                ..
            } => todo!("parse super"),
            Token {
                kind: TokenType::This,
                ..
            } => todo!("parse this "),
            Token {
                kind: TokenType::Return,
                ..
            } => todo!("parse return "),
            Token {
                kind: TokenType::Else,
                ..
            } => todo!("parse Else"),
            Token {
                kind: TokenType::Class,
                ..
            } => todo!("parse class"),
            Token {
                kind: TokenType::For,
                ..
            } => {
                if self.expect(TokenType::LeftParen) {
                    let var = self.parse_expr(0)?;
                    if !self.expect_semicolon() {
                        return Err(miette!("expected a ; to start the for loop"));
                    }
                    let cond = self.parse_expr(0)?;
                    if !self.expect_semicolon() {
                        return Err(miette!("expected a ; to start the for loop"));
                    }
                    let inc = self.parse_expr(0)?;
                    let block = self.parse_block()?;
                    return Ok(Tree::NonTerm(Op::For, vec![var, cond, inc, block]));
                } else {
                    return Err(miette!("expected a ( to start the for loop"));
                };
            }
            Token {
                kind: TokenType::While,
                ..
            } => todo!("parse while"),
            Token {
                kind: TokenType::If,
                ..
            } => todo!("parse if"),
            Token {
                kind: TokenType::LeftBrace,
                ..
            } => todo!("parse block"),
            _ => todo!("cant parse"),
        };
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

fn infix_binding_power(op: Op) -> Option<(u8, u8)> {
    let res = match op {
        // '=' => (2, 1),
        // '?' => (4, 3),
        Op::And | Op::Or => (3, 4),
        Op::BangEqual
        | Op::EqualEqual
        | Op::Less
        | Op::LessEqual
        | Op::Greater
        | Op::GreaterEqual => (5, 6),
        Op::Plus | Op::Minus => (7, 8),
        Op::Star | Op::Slash => (9, 10),
        Op::Field => (16, 15),
        _ => return None,
    };
    Some(res)
}

fn postfix_binding_power(op: Op) -> Option<(u8, ())> {
    let res = match op {
        Op::Call => (13, ()),
        _ => return None,
    };
    Some(res)
}

impl std::fmt::Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Op::Minus => "-",
                Op::Plus => "+",
                Op::Star => "*",
                Op::BangEqual => "!=",
                Op::EqualEqual => "==",
                Op::LessEqual => "<=",
                Op::GreaterEqual => ">=",
                Op::Less => "<",
                Op::Greater => ">",
                Op::Slash => "/",
                Op::Bang => "!",
                Op::And => "and",
                Op::Or => "or",
                Op::For => "for",
                Op::Class => "class",
                Op::Print => "print",
                Op::Return => "return",
                Op::Field => ".",
                Op::Var => "var",
                Op::While => "while",
                Op::Call => "call",
                Op::Group => "group",
            }
        )
    }
}

impl<'a> std::fmt::Display for Tree<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tree::Atom(x) => {
                write!(f, "{:?}", x)
            }
            Tree::NonTerm(parent, children) => {
                write!(f, "({}", parent)?;
                for i in children {
                    write!(f, ", {}", i)?
                }
                write!(f, ")")
            }
            _ => {
                println!("hello");
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
