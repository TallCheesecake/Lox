use miette::{Diagnostic, Error, Result, SourceSpan};
use std::collections::HashMap;
use std::fmt::Display;
use std::ops::Range;
use std::str::Chars;

#[derive(Debug, Diagnostic)]
#[diagnostic(help("This is most likely a invalid token (some char not allowed in the language)"))]
pub struct MyBad {
    #[source_code]
    source: String,
    #[label("main issue")]
    primary_span: SourceSpan,
}

impl std::error::Error for MyBad {}
impl std::fmt::Display for MyBad {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid token found")
    }
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
#[derive(PartialEq, Debug, Copy, Clone)]
pub enum TokenType {
    LeftParen,
    RightParen,
    LeftHardBrace,
    RightHardBrace,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number(f64),

    // Keywords.
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

#[derive(Clone, Debug)]
pub struct Token {
    pub kind: TokenType,
    pub range: Range<usize>,
}

//TODO: make the eof function a method of scnner
// impl<'a> Display for Token<'a> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", &self.lexeme)
//     }
// }

pub struct Scanner<'a> {
    keywords: HashMap<&'a str, TokenType>,
    code: &'a str,
    current: usize,
    start: usize,
    chars: Chars<'a>,
    line: usize,
}

impl<'a> Iterator for Scanner<'a> {
    type Item = miette::Result<Token, miette::Report>;
    fn next(&mut self) -> Option<Self::Item> {
        pub enum ThirdState {
            OrEquals(char),
            String,
            Number,
            Iden,
        }
        //TODO: make 2 error types
        let generate =
            |kind: TokenType, end: usize, start: usize| -> Option<Result<Token, Error>> {
                Some(Ok(Token {
                    kind,
                    range: Range { start, end },
                }))
            };

        self.handle_whitespace();
        self.current = self.code[self.chars.as_str().len()..].len();
        self.start = self.current;
        let c = self.chars.next()?;
        let third_state = match c {
            '(' => {
                self.current += c.len_utf8();
                return generate(TokenType::LeftParen, self.current, self.start);
            }
            ')' => {
                self.current += c.len_utf8();
                return generate(TokenType::RightParen, self.current, self.start);
            }
            '[' => {
                self.current += c.len_utf8();
                return generate(TokenType::LeftHardBrace, self.current, self.start);
            }
            ']' => {
                self.current += c.len_utf8();
                return generate(TokenType::RightHardBrace, self.current, self.start);
            }
            '{' => {
                self.current += c.len_utf8();
                return generate(TokenType::LeftBrace, self.current, self.start);
            }
            '}' => {
                self.current += c.len_utf8();
                return generate(TokenType::RightBrace, self.current, self.start);
            }
            ';' => {
                self.current += c.len_utf8();
                return generate(TokenType::Semicolon, self.current, self.start);
            }
            ',' => {
                self.current += c.len_utf8();
                return generate(TokenType::Comma, self.current, self.start);
            }
            '.' => {
                self.current += c.len_utf8();
                return generate(TokenType::Dot, self.current, self.start);
            }
            '-' => {
                self.current += c.len_utf8();
                return generate(TokenType::Minus, self.current, self.start);
            }
            '+' => {
                self.current += c.len_utf8();
                return generate(TokenType::Plus, self.current, self.start);
            }
            '/' => {
                self.current += c.len_utf8();
                return generate(TokenType::Slash, self.current, self.start);
            }
            '*' => {
                self.current += c.len_utf8();
                return generate(TokenType::Star, self.current, self.start);
            }
            '\0' => {
                // println!("eof");
                return generate(TokenType::Eof, self.current, self.start);
            }
            '!' | '=' | '<' | '>' => ThirdState::OrEquals(c),
            'A'..='Z' | 'a'..='z' => ThirdState::Iden,
            '0'..='9' => ThirdState::Number,
            '"' => ThirdState::String,
            _ => {
                return Some(Err(MyBad {
                    source: self.code.into(),
                    primary_span: SourceSpan::new(self.start.into(), self.lexeme().len().into()),
                }
                .into()));
            }
        };

        let mut comparator_handle =
            |kind: TokenType, kind2: TokenType| -> Option<Result<Token, Error>> {
                {
                    if self.first() == '=' {
                        self.chars.next()?;
                        self.current = self.code[self.chars.as_str().len()..].len();
                        return generate(kind, self.current, self.start);
                    } else {
                        self.current = self.code[self.chars.as_str().len()..].len();
                        return generate(kind2, self.current, self.start);
                    }
                }
            };
        match third_state {
            ThirdState::OrEquals(c) => match c {
                '<' => {
                    return comparator_handle(TokenType::LessEqual, TokenType::Less);
                }
                '!' => {
                    return comparator_handle(TokenType::BangEqual, TokenType::Bang);
                }
                '>' => {
                    return comparator_handle(TokenType::GreaterEqual, TokenType::Greater);
                }
                '=' => {
                    return comparator_handle(TokenType::EqualEqual, TokenType::Equal);
                }
                _ => {
                    return Some(Err(MyBad {
                        source: self.code.into(),
                        primary_span: SourceSpan::new(
                            self.start.into(),
                            self.lexeme().len().into(),
                        ),
                    }
                    .into()));
                }
            },

            ThirdState::String => {
                if let Some(c) = self.chars.find(|&x| x == '"') {
                    if c == '"' {
                        self.current = self.code[self.chars.as_str().len()..].len();
                        return generate(TokenType::String, self.current, self.start);
                    } else {
                        return generate(TokenType::String, self.current, self.start);
                    }
                }
            }

            ThirdState::Number => {
                while self.first().is_ascii_digit() {
                    self.chars.next()?;
                }
                if self.first() == '.' && self.second().is_ascii_digit() {
                    self.chars.next()?;

                    while self.first().is_ascii_digit() {
                        self.chars.next()?;
                    }
                }

                self.current = self.code[self.chars.as_str().len()..].len();
                match self.lexeme().parse::<f64>() {
                    Ok(x) => {
                        return generate(TokenType::Number(x), self.current, self.start);
                    }
                    Err(x) => eprintln!("{}", x),
                }
            }

            ThirdState::Iden => {
                while self.first().is_ascii_alphanumeric() {
                    self.chars.next();
                }
                self.current = self.code[self.chars.as_str().len()..].len();
                let index = self.lexeme();
                // println!("lexem: {index}");
                if let Some(c) = self.keywords.get_mut(&(index)) {
                    return generate(*c, self.current, self.start);
                } else {
                    return generate(TokenType::Identifier, self.current, self.start);
                }
            }
        };
        None
    }
}

impl<'a> Scanner<'a> {
    pub fn new(code: &'a str) -> Scanner<'a> {
        let mut keywords = HashMap::with_capacity(16);
        keywords.insert("else", TokenType::Else);
        keywords.insert("and", TokenType::And);
        keywords.insert("for", TokenType::For);
        keywords.insert("fun", TokenType::Fun);
        keywords.insert("class", TokenType::Class);
        keywords.insert("return", TokenType::Return);
        keywords.insert("if", TokenType::If);
        keywords.insert("nil", TokenType::Nil);
        keywords.insert("or", TokenType::Or);
        keywords.insert("true", TokenType::True);
        keywords.insert("false", TokenType::False);
        keywords.insert("print", TokenType::Print);
        keywords.insert("super", TokenType::Super);
        keywords.insert("this", TokenType::This);
        keywords.insert("var", TokenType::Var);
        keywords.insert("while", TokenType::While);

        Scanner {
            keywords,
            code,
            current: 0,
            start: 0,
            chars: code.chars(),
            line: 1,
        }
    }
    fn handle_whitespace(&mut self) {
        while matches!(self.first(), '\n' | ' ') {
            if self.first() == '\n' {
                self.line += 1;
            };
            self.chars.next().unwrap_or('\0');
            self.current = self.code[self.chars.as_str().len()..].len();
        }
        self.current = self.code[self.chars.as_str().len()..].len();
    }

    fn lexeme(&mut self) -> &'a str {
        &self.code[self.start..self.current]
    }

    fn second(&self) -> char {
        let mut temp = self.chars.clone();
        temp.next();
        temp.next().unwrap_or('\0')
    }
    fn first(&self) -> char {
        let mut temp = self.chars.clone();
        temp.next().unwrap_or('\0')
    }
}

pub fn collect(input: &str) -> Result<Vec<Token>, miette::Report> {
    let mut iter = Scanner::new(input).into_iter();
    let mut tokens = Vec::new();
    while let Some(res) = iter.next() {
        let token = res?;
        tokens.push(token);
    }
    Ok(tokens)
}
