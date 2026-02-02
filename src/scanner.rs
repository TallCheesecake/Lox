use miette::{Diagnostic, IntoDiagnostic, Result, SourceSpan};
use std::collections::HashMap;
use std::fmt::{Display, write};
use std::str::Chars;

#[derive(Debug, Diagnostic)]
#[diagnostic(help("try doing it better next time?"))]
pub struct MyBad {
    #[source_code]
    pub source: String,
    #[label("main issue")]
    pub primary_span: SourceSpan,
}

impl std::error::Error for MyBad {}

impl std::fmt::Display for MyBad {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "error around: {} ", self.source)
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum TokenType {
    LeftParen,
    RightParen,
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

#[derive(Copy, Clone, Debug)]
pub struct Token<'a> {
    pub kind: TokenType,
    pub line: usize,
    pub lexeme: &'a str,
}
//TODO: make the eof function a method of scnner
impl<'a> Display for Token<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.lexeme)
    }
}

pub struct Scanner<'a> {
    keywords: HashMap<&'a str, TokenType>,
    code: &'a str,
    current: usize,
    start: usize,
    chars: Chars<'a>,
    line: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(code: &'a str) -> Scanner {
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
        let mut i = 0;
        while matches!(self.first(), ' ') {
            i += 1;
            eprintln!("called next : {} times", i);
            self.chars.next().unwrap_or('\0');
        }
        self.current = self.code[self.chars.as_str().len()..].len();
    }
    pub fn generator(&mut self) -> Option<Result<Token<'a>, MyBad>> {
        pub enum ThirdState {
            OrEquals(char),
            String,
            Number,
            Iden,
        }
        //TODO: make 2 error types
        let generate =
            |kind: TokenType, line: usize, lexeme: &'a str| -> Option<Result<Token<'a>, MyBad>> {
                Some(Ok(Token { kind, line, lexeme }))
            };
        //call before self.start (whitespace)
        self.handle_whitespace();
        self.start = self.current;
        let mut counter: usize = 0;
        let c = self.chars.next().unwrap_or('\0');
        if c == '\n' {
            counter += 1;
        }
        let third_state = match c {
            '(' => {
                self.current = c.len_utf8();
                return generate(TokenType::LeftParen, counter, self.lexeme());
            }
            ')' => {
                self.current = c.len_utf8();
                return generate(TokenType::RightParen, counter, self.lexeme());
            }
            '{' => {
                self.current = c.len_utf8();
                return generate(TokenType::LeftBrace, counter, self.lexeme());
            }
            '}' => {
                self.current = c.len_utf8();
                return generate(TokenType::RightBrace, counter, self.lexeme());
            }
            ';' => {
                self.current = c.len_utf8();
                return generate(TokenType::Semicolon, counter, self.lexeme());
            }
            ',' => {
                self.current = c.len_utf8();
                return generate(TokenType::Comma, counter, self.lexeme());
            }
            '.' => {
                self.current = c.len_utf8();
                return generate(TokenType::Dot, counter, self.lexeme());
            }
            '-' => {
                self.current = c.len_utf8();
                return generate(TokenType::Minus, counter, self.lexeme());
            }
            '+' => {
                self.current = c.len_utf8();
                return generate(TokenType::Plus, counter, self.lexeme());
            }
            '/' => {
                self.current = c.len_utf8();
                return generate(TokenType::Slash, counter, self.lexeme());
            }
            '*' => {
                self.current = c.len_utf8();
                return generate(TokenType::Star, counter, self.lexeme());
            }
            '!' | '=' | '<' | '>' => ThirdState::OrEquals(c),
            'A'..='Z' | 'a'..='z' => ThirdState::Iden,
            '0'..='9' => ThirdState::Number,
            '"' => ThirdState::String,
            _ => {
                return Some(Err(MyBad {
                    source: self.code.into(),
                    primary_span: SourceSpan::new(0.into(), self.lexeme().len().into()),
                }));
            }
        };

        let mut comparator_handle =
            |kind: TokenType, kind2: TokenType| -> Option<Result<Token<'a>, MyBad>> {
                {
                    if self.first() == '=' {
                        self.chars.next()?;
                        self.current = self.code[self.chars.as_str().len()..].len();
                        return generate(kind, counter, self.lexeme());
                    } else {
                        self.current = self.code[self.chars.as_str().len()..].len();
                        return generate(kind2, counter, self.lexeme());
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
                        primary_span: SourceSpan::new(0.into(), self.lexeme().len().into()),
                    }));
                }
            },

            ThirdState::String => {
                if let Some(c) = self.chars.find(|&x| x == '"') {
                    if c == '"' {
                        self.current = self.code[self.chars.as_str().len()..].len();
                        return generate(TokenType::String, counter, self.lexeme());
                    } else {
                        return generate(TokenType::String, counter, self.lexeme());
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
                        return generate(TokenType::Number(x), counter, self.lexeme());
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
                if let Some(c) = self.keywords.get_mut(&(index)) {
                    return generate(*c, counter, self.lexeme());
                } else {
                    return generate(TokenType::Identifier, counter, self.lexeme());
                    //
                }
            }
        };
        None
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

#[cfg(test)]
mod tests {
    #[test]
    use super::*;
    fn number_one() {
        let mut scanner = Scanner::new("1");

        let token = scanner
            .generator()
            .expect("expected a token")
            .expect("lexer returned error");

        eprintln!("TOKEN VALUE: {}", token);
        assert_eq!(token.kind, TokenType::Number(1 as f64));
        assert_eq!(token.lexeme, "1");
    }
}
