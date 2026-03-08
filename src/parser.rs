//TODO: Parse expresion statments properly
use crate::scanner::{self, Token, TokenType};
use core::panic;
use miette::{Context, Diagnostic, Result, SourceSpan};
use std::ops::Range;
use std::rc::Rc;
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

#[derive(Debug)]
pub enum Tree {
    Nil,
    ExprStatment(Vec<Tree>),
    Var(String, Rc<Tree>),
    Call {
        callee: Box<Tree>,
        arguments: Vec<Tree>,
    },
    Atom(Atom),
    NonTerm(Op, Vec<Tree>),
    Op(Op),
    Fun {
        name: Rc<Tree>,
        parameters: Vec<Tree>,
        body: Rc<Tree>,
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
            TokenType::Equal => Op::Equal,
            TokenType::LessEqual => Op::LessEqual,
            TokenType::And => Op::And,
            TokenType::Class => Op::Class,
            TokenType::Or => Op::Or,
            TokenType::Print => Op::Print,
            TokenType::Return => Op::Return,
            TokenType::Var => Op::Var,
            TokenType::While => Op::While,
            TokenType::If => Op::If,
            TokenType::Else => Op::Else,
            _ => unreachable!(),
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Op {
    Minus,
    If,
    Else,
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
    Equal,
}

#[derive(Debug)]
pub struct Parser {
    pub stream: Vec<Token>,
    pub input: Arc<String>,
    pub pos: usize,
}

impl Parser {
    pub fn new(input: String) -> Result<Parser, miette::Report> {
        let stream = scanner::collect(&input)?;

        Ok(Parser {
            stream,
            input: Arc::new(input),
            pos: 0,
        })
    }

    pub fn parse_program(&mut self) -> Result<Vec<Tree>, miette::Report> {
        let mut parent = Vec::new();
        while self.pos <= self.stream.len() {
            let val = self.parse_statment()?;
            match val {
                Tree::Atom(x) => match x {
                    Atom::Nil => {
                        break;
                    }
                    _ => continue,
                },
                x => parent.push(x),
            }
        }
        Ok(parent)
    }

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
                kind: TokenType::Minus | TokenType::Print | TokenType::Bang,
                ..
            } => {
                let ((), bp) = prefix_binding_power(n.kind);
                let rhs = self.parse_expr(bp)?;
                Tree::NonTerm(n.kind.into(), vec![rhs])
            }
            Token {
                kind: TokenType::LeftParen | TokenType::LeftBrace,
                ..
            } => {
                let lhs = self.parse_expr(0)?;
                if self.expect(TokenType::RightParen) | self.expect(TokenType::RightBrace) {
                    return self.error("Expected either ) or } ");
                };
                Tree::NonTerm(Op::Group, vec![lhs])
            }

            _ => {
                return self.error("Expected expresion");
            }
        };

        loop {
            let op = match self.peek() {
                Token {
                    kind:
                        TokenType::Eof
                        | TokenType::RightParen
                        | TokenType::RightBrace
                        | TokenType::Comma
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
                        | TokenType::BangEqual
                        | TokenType::EqualEqual
                        | TokenType::Minus
                        | TokenType::GreaterEqual
                        | TokenType::Equal
                        | TokenType::Greater
                        | TokenType::LessEqual
                        | TokenType::Less
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
                    let rhs = self.parse_expr(0)?;
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
            break;
        }
        Ok(lhs)
    }
    fn error(&mut self, msg_input: &str) -> Result<Tree, miette::Report> {
        let msg = format!("{}", msg_input);
        return Err(ParserError {
            source: Arc::clone(&self.input),
            primary_span: token_to_span(&self.current()),
        })
        .wrap_err(msg);
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

    pub fn parse_statment(&mut self) -> Result<Tree, miette::Report> {
        let lhs = match self.peek() {
            Token {
                kind: TokenType::Fun,
                ..
            } => {
                self.advance();
                if !self.expect(TokenType::Identifier) {
                    return self.error("Expected function name");
                };

                let range = self.advance().range;
                let func_iden = Tree::Atom(Atom::Ident {
                    range,
                    source: Arc::clone(&self.input),
                });

                let mut temp = Vec::new();
                if !self.expect(TokenType::LeftParen) {
                    return self.error("Expected (");
                };
                self.advance();
                loop {
                    if self.expect(TokenType::RightParen) {
                        break;
                    }
                    if self.expect(TokenType::Comma) {
                        self.advance();
                    }
                    let range = self.current().range;
                    let attempt: Tree = self.parse_expr(0).unwrap_or_else(|_| {
                        self.advance();
                        Tree::Atom(Atom::Ident {
                            range,
                            source: Arc::clone(&self.input),
                        })
                    });
                    temp.push(attempt);
                }
                if !self.expect(TokenType::RightParen) {
                    return self.error("Expected )");
                };
                self.advance();
                let block = self.parse_statment()?;
                return Ok(Tree::Fun {
                    name: Rc::new(func_iden),
                    parameters: temp,
                    body: Rc::new(block),
                });
            }

            Token {
                kind: TokenType::Super,
                ..
            } => todo!("parse super"),
            Token {
                kind: TokenType::Var,
                ..
            } => {
                self.advance();
                if !self.expect(TokenType::Identifier) {
                    return self.error("You must add a identifier after var");
                };
                let iden = self.advance();
                if !self.expect(TokenType::Equal) {
                    return self.error("You must add a = after iden");
                };
                self.advance();
                let val = self.parse_expr(0)?;
                if !self.expect_semicolon() {
                    return self.error("Expected ;");
                };
                let name = String::from(&self.input[iden.range.start..iden.range.end]);
                self.advance();
                let temp = Rc::new(val);
                Tree::Var(name, temp)
            }

            Token {
                kind: TokenType::Else,
                ..
            } => {
                self.advance();
                let block = self.parse_statment()?;
                return Ok(Tree::NonTerm(Op::Else, vec![block]));
            }

            Token {
                kind: TokenType::Class,
                ..
            } => {
                self.advance();
                if !self.expect(TokenType::Identifier) {
                    return self.error("Expected ;");
                };

                self.advance();
                let cls_name = Tree::Atom(Atom::Ident {
                    range: Range {
                        start: self.current().range.start,
                        end: self.current().range.end,
                    },
                    source: Arc::clone(&self.input),
                });

                if !self.expect(TokenType::LeftBrace) {
                    return self.error("Expected ;");
                };

                let inside = self.parse_statment()?;
                return Ok(Tree::NonTerm(Op::Class, vec![cls_name, inside]));
            }

            Token {
                kind: TokenType::For,
                ..
            } => {
                self.advance();
                if self.expect(TokenType::LeftParen) {
                    self.advance();
                    let var = self.parse_statment()?;
                    let cond = self.parse_statment()?;
                    let inc = self.parse_statment()?;
                    if !self.expect(TokenType::RightParen) {
                        return self.error("Expected a )");
                    };
                    self.advance();
                    let block = self.parse_statment()?;
                    return Ok(Tree::NonTerm(Op::For, vec![var, cond, inc, block]));
                } else {
                    return self.error("Expected a (");
                };
            }
            Token {
                kind: TokenType::While,
                ..
            } => {
                self.advance();
                if !self.expect(TokenType::LeftParen) {
                    return self.error("Expected a (");
                };
                self.advance();
                let cond = self.parse_statment()?;
                if !self.expect(TokenType::RightParen) {
                    return self.error("Expected a )");
                };
                self.advance();
                if !self.expect(TokenType::LeftBrace) {
                    return self.error("Expected a {");
                };
                let block = self.parse_statment()?;
                Tree::NonTerm(Op::While, vec![cond, block])
            }
            Token {
                kind: TokenType::If,
                ..
            } => {
                self.advance();
                if !self.expect(TokenType::LeftParen) {
                    return self.error("Expected a (");
                };
                self.advance();
                let cond = self.parse_expr(0)?;
                if !self.expect(TokenType::RightParen) {
                    return self.error("Expected a )");
                };
                self.advance();
                if !self.expect(TokenType::LeftBrace) {
                    return self.error("Expected a {");
                };
                let block = self.parse_statment()?;
                Tree::NonTerm(Op::If, vec![cond, block])
            }
            Token {
                kind: TokenType::LeftBrace,
                ..
            } => {
                let mut child = Vec::new();
                self.advance();
                while !self.expect(TokenType::RightBrace) {
                    let val = self.parse_statment()?;
                    match val {
                        Tree::Atom(x) => match x {
                            Atom::Nil => {
                                break;
                            }
                            _ => continue,
                        },
                        x => {
                            child.push(x);
                            println!("val: {:?}", child.len());
                        }
                    }
                }
                if !self.expect(TokenType::RightBrace) {
                    return self.error("Expected } ");
                };
                self.advance();
                return Ok(Tree::NonTerm(Op::Group, child));
            }
            _ => {
                if self.expect(TokenType::Eof) {
                    return Ok(Tree::Atom(Atom::Nil));
                }
                let val = self.parse_expr(0)?;
                if self.expect(TokenType::Semicolon) {
                    self.advance();
                    let mut temp: Vec<Tree> = Vec::new();
                    temp.push(val);
                    return Ok(Tree::ExprStatment(temp));
                }
                return Ok(val);
            }
        };
        Ok(lhs)
    }
}

fn prefix_binding_power(op: scanner::TokenType) -> ((), u8) {
    match op {
        TokenType::Plus | TokenType::Minus | TokenType::Print => ((), 5),
        _ => {
            panic!("woops bad token this should be a error")
        }
    }
}

fn infix_binding_power(op: scanner::TokenType) -> Option<(u8, u8)> {
    let res = match op.into() {
        Op::Equal => (2, 1),
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
                Op::Equal => "=",
                Op::Plus => "+",
                Op::Star => "*",
                Op::BangEqual => "!=",
                Op::EqualEqual => "==",
                Op::LessEqual => "<=",
                Op::GreaterEqual => ">=",
                Op::Less => "<",
                Op::Greater => ">",
                Op::If => "if",
                Op::Else => "else",
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
            Tree::ExprStatment(op) => {
                write!(f, "(")?;

                for i in op {
                    write!(f, ", {}", i)?
                }
                write!(f, ")")
            }
            Tree::Op(op) => {
                write!(f, "{}", op)
            }
            Tree::Nil => {
                write!(f, "EOF")
            }
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
                write!(f, "(")?;
                write!(f, "k: {}", k)?;
                write!(f, "{:?}", x)?;
                write!(f, ")")
            }
            _ => unimplemented!(),
        }
    }
}
