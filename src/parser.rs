use crate::scanner::TokenType;
use crate::scanner::{self, Token};
use core::panic;
use miette::{Context, Diagnostic, Result, SourceSpan};
use miette::{Error, LabeledSpan, miette};
use std::any::Any;
use std::collections::HashMap;
use std::io::Stdout;
use std::ops::Range;
use std::panic::PanicInfo;
use std::sync::Arc;

#[derive(Debug, Diagnostic)]
#[diagnostic(help("this is a syntax error"))]
pub struct ParserError {
    #[source_code]
    source: Arc<String>,
    #[label("main issue")]
    primary_span: SourceSpan,
}

impl std::error::Error for ParserError {}
impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid or missing token")
    }
}
fn token_to_span(token: &Token) -> SourceSpan {
    SourceSpan::new(
        token.range.start.into(),
        (token.range.end - token.range.start).into(),
    )
}
// struct Scope {
//     parent: Box<Option<Scope>>,
//     parent: Box<Option<Scope>>,
// }
#[derive(Debug)]
pub enum Tree {
    Nil,
    Var(String, HashMap<String, Tree>),
    Call {
        callee: Box<Tree>,
        arguments: Vec<Tree>,
    },
    Atom(Atom),
    NonTerm(Op, Vec<Tree>),
    Op(Op),
    Fun {
        name: Box<Tree>,
        parameters: Vec<Tree>,
        body: Box<Tree>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Atom {
    Ident {
        range: Range<usize>,
        source: Arc<String>,
    },
    String {
        range: Range<usize>,
        source: Arc<String>,
    },
    Number(f64),
    Nil,
    Bool(bool),
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
pub struct Parser {
    pub stream: Vec<Token>,
    pub input: Arc<String>,
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
impl Parser {
    pub fn new(input: String) -> Result<Parser, miette::Report> {
        let stream = scanner::collect(&input)?;

        Ok(Parser {
            stream,
            input: Arc::new(input),
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
    pub fn parse_expr(&mut self, min_bp: u8) -> Result<Tree, miette::Report> {
        let mut lhs = match self.advance() {
            Token {
                kind: TokenType::String,
                range: Range { start, end },
            } => Tree::Atom(Atom::String {
                range: Range { start, end },
                source: Arc::clone(&self.input),
            }),
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
                range: Range { start, end },
            } => Tree::Atom(Atom::Ident {
                range: Range { start, end },
                source: Arc::clone(&self.input),
            }),
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
                if self.expect(TokenType::RightParen) {
                    return Err(ParserError {
                        source: Arc::clone(&self.input),
                        primary_span: token_to_span(&self.current()),
                    })
                    .wrap_err("I dont even know how you got here")
                    .into();
                }
                Tree::NonTerm(Op::Group, vec![lhs])
            }
            _ => {
                return Err(ParserError {
                    source: Arc::clone(&self.input),
                    primary_span: token_to_span(&self.current()),
                })
                .wrap_err("Expected expresion")
                .into();
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
                    kind:
                        TokenType::Plus
                        | TokenType::LeftParen
                        | TokenType::LeftBrace
                        | TokenType::LeftHardBrace
                        | TokenType::Minus
                        | TokenType::Less
                        | TokenType::LessEqual
                        | TokenType::Slash
                        | TokenType::Star,
                    ..
                } => n,
                _ => {
                    break;
                }
            };

            if let Some((l_bp, ())) = postfix_binding_power(op.kind) {
                if l_bp < min_bp {
                    break;
                }
                self.advance();
                lhs = if op.kind == TokenType::LeftHardBrace {
                    // let temp = op.lexeme.to_string();
                    let rhs = self.parse_expr(0)?;
                    // if self.peek().kind != TokenType::RightHardBrace {
                    //     panic!();
                    //     let source = String::from(format!("{temp} ... EXPECTING CLOSING: ]"));
                    //     return Err(miette!(
                    //         severity = Severity::Error,
                    //         help = "hello is a syntax error",
                    //         labels = vec![LabeledSpan::at_offset(0, "here")],
                    //         "Missing Closing ]"
                    //     )
                    //     .with_source_code(source));
                    // }

                    Tree::NonTerm(op.kind.into(), vec![lhs, rhs])
                } else {
                    Tree::NonTerm(op.kind.into(), vec![lhs])
                };
                continue;
            }

            //INFIX
            if let Some((l_bp, r_bp)) = infix_binding_power(op.kind) {
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

    pub fn advance(&mut self) -> Token {
        let output = self.stream.get(self.pos).expect("Invariant broken: should not be possible for advance to return none since the prev match should break out on EOF.").clone();
        self.pos += 1;
        output
    }

    pub fn current(&mut self) -> Token {
        self.stream
            .get(self.pos - 1)
            .expect("Invariant broken: if current reached None that means that is bad.")
            .clone()
    }

    // behave like peek()
    pub fn peek(&mut self) -> Token {
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
    pub fn parse_print(&mut self) -> Result<Tree, miette::Report> {
        let value = self.peek();
        // println!("{:?}", value);
        if value.kind != TokenType::String {
            return Err(ParserError {
                source: Arc::clone(&self.input),
                primary_span: token_to_span(&self.current()),
            })
            .wrap_err("dont even know how you got here")
            .into();
        } else if !self.expect_semicolon() {
            let val = self.advance();
            let range = val.range;
            return Ok(Tree::Atom(Atom::Ident {
                range,
                source: Arc::clone(&self.input),
            }));
        } else {
            panic!("need to make error")
            // let source = String::from(v);
            // return Err(miette!(
            //     severity = Severity::Error,
            //     help = "This is a syntax error",
            //     labels = vec![LabeledSpan::at_offset(value.lexeme.len() - 1, "here")],
            //     "Missing Semicolon ';'"
            // )
            // .with_source_code(source));
        };
    }

    fn expect(&mut self, input: TokenType) -> bool {
        if self.peek().kind == input {
            true
        } else {
            false
        }
    }

    fn expect_semicolon(&mut self) -> bool {
        if self.peek().kind == TokenType::Semicolon {
            true
        } else {
            false
        }
    }
    pub fn parse_block(&mut self) -> Result<Tree, miette::Report> {
        if self.expect(TokenType::LeftBrace) {
            self.advance();
            let middle = self.parse_expr(0)?;
            if self.expect(TokenType::RightBrace) {
                return Ok(Tree::NonTerm(Op::Group, vec![middle]));
            } else {
                return Err(ParserError {
                    source: Arc::clone(&self.input),
                    primary_span: token_to_span(&self.current()),
                })
                .wrap_err("Expected '}' ");
            }
        } else {
            self.advance();
            return Err(ParserError {
                source: Arc::clone(&self.input),
                primary_span: token_to_span(&self.current()),
            })
            .wrap_err("Expected '{' ");
        };
    }
    pub fn parse_statment(&mut self) -> Result<Tree, miette::Report> {
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
                    return Err(ParserError {
                        source: Arc::clone(&self.input),
                        primary_span: token_to_span(&self.current()),
                    })
                    .wrap_err("Expected a funciton name");
                } else {
                    let range = self.advance().range;
                    let func_iden = Tree::Atom(Atom::Ident {
                        range,
                        source: Arc::clone(&self.input),
                    });
                    let mut temp = Vec::new();
                    if !self.expect(TokenType::LeftParen) {
                        return Err(ParserError {
                            source: Arc::clone(&self.input),
                            primary_span: token_to_span(&self.current()),
                        }
                        .into());
                    } else {
                        self.advance();
                        if self.expect(TokenType::RightParen) {
                            self.advance();
                            {}
                        } else {
                            loop {
                                if self.expect(TokenType::Comma) {
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
                kind: TokenType::Var,
                ..
            } => {
                //TODO: put this is a field
                let mut map = HashMap::new();
                if self.expect(TokenType::Identifier) {
                    let iden = self.advance();
                    if self.expect(TokenType::Equal) {
                        self.advance();
                        let val = self.parse_expr(0)?;
                        let key = String::from(&self.input[iden.range.start..=iden.range.end]);
                        map.insert(key.clone(), val);
                        Tree::Var(key, map)
                    } else {
                        return Err(ParserError {
                            source: Arc::clone(&self.input),
                            primary_span: token_to_span(&self.current()),
                        })
                        .wrap_err("Please add a = sign after your var identintifier")
                        .into();
                    }
                } else {
                    return Err(ParserError {
                        source: Arc::clone(&self.input),
                        primary_span: token_to_span(&self.current()),
                    })
                    .wrap_err("You must add a identifier after var")
                    .into();
                }
            }
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
                    self.advance();
                    let var = self.parse_statment()?;
                    if !self.expect_semicolon() {
                        return Err(ParserError {
                            source: Arc::clone(&self.input),
                            primary_span: token_to_span(&self.current()),
                        })
                        .wrap_err("Expected a ';' for a post var declaration");
                    }
                    self.advance();
                    let cond = self.parse_expr(0)?;
                    if !self.expect_semicolon() {
                        return Err(ParserError {
                            source: Arc::clone(&self.input),
                            primary_span: token_to_span(&self.current()),
                        })
                        .wrap_err("Expected a ';' for a post cond declaration");
                    }
                    self.advance();
                    let inc = self.parse_expr(0)?;
                    if !self.expect(TokenType::RightParen) {
                        return Err(ParserError {
                            source: Arc::clone(&self.input),
                            primary_span: token_to_span(&self.current()),
                        })
                        .wrap_err("Expected a ')'");
                    }
                    self.advance();
                    println!("prev blcok {:?}", self.peek());
                    let block = self.parse_block()?;
                    //TODO: That cant be how this is done
                    return Ok(Tree::NonTerm(Op::For, vec![var, cond, inc, block]));
                } else {
                    return Err(ParserError {
                        source: Arc::clone(&self.input),
                        primary_span: token_to_span(&self.current()),
                    })
                    .wrap_err("Expected a '(' for a for loop");
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

fn infix_binding_power(op: scanner::TokenType) -> Option<(u8, u8)> {
    let res = match op.into() {
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

fn postfix_binding_power(op: scanner::TokenType) -> Option<(u8, ())> {
    let res = match op.into() {
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

impl std::fmt::Display for Atom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Atom::Ident { range, source } => write!(f, "{}", &source[range.start..range.end]),
            Atom::String { range, source } => write!(f, "{}", &source[range.start..range.end]),
            Atom::Number(x) => write!(f, "{x}"),
            Atom::Nil => write!(f, "Nil"),
            Atom::Bool(x) => write!(f, "{x}"),
            Atom::This => write!(f, "This"),
            Atom::Super => write!(f, "Super"),
            Atom::Error => write!(f, "Error"),
        }
    }
}

impl std::fmt::Display for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tree::Atom(x) => {
                write!(f, "{}", x)
            }

            Tree::NonTerm(parent, children) => {
                write!(f, "({}", parent)?;

                for i in children {
                    write!(f, ", {}", i)?
                }
                write!(f, ")")
            }

            Tree::Fun {
                name,
                parameters,
                body,
            } => {
                write!(f, "{}", name)?;
                write!(f, "(")?;
                for i in parameters {
                    write!(f, " {} ", i)?
                }
                write!(f, ")")?;
                write!(f, " {{ {} }}", body)
            }

            Tree::Var(k, x) => {
                let iden = x.get(k).unwrap();
                write!(f, "Var:  K: {} V: {} ", k, iden)
            }
            _ => unimplemented!(),
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
